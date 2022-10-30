use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Weak,
    },
    task::{Context, Poll},
    time::{Duration, Instant},
};

use dashmap::DashMap;
use log::{error, trace, warn};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{
        mpsc::{self, error::SendTimeoutError},
        oneshot,
    },
};

use crate::{
    proto::{packet::PacketBody, CommandId, Packet, ResponseCode},
    RequestHandler, ResponseHandle, ResponseHandleOwned,
};
use anyhow::{bail, Result};
use prost::Message;

pub struct Connection {
    next_local_id: AtomicU64,
    socket_addr: SocketAddr,
    outbound_channel: mpsc::Sender<Packet>,
    pending_responses: Arc<DashMap<u64, oneshot::Sender<ProcessedPacketOwned>>>,
}

// 10 MB
const MAX_PACKET_LENGTH: u64 = 1024 * 1024 * 10;
const CHANNEL_DEPTH: usize = 10;

async fn read_packet(mut input: impl AsyncRead + Unpin) -> Result<Packet> {
    let length: u64 = input.read_u64().await?;
    if length > MAX_PACKET_LENGTH {
        bail!("length of inbound packet is too high: {length} > {MAX_PACKET_LENGTH}");
    }
    let length: usize = length.try_into().expect("u64 too big for usize");
    let mut bytes: Vec<u8> = Vec::with_capacity(length);
    {
        // this strips lifetime, so must be careful here
        // we use an extra scope to enforce bytes_target is not used again
        let bytes_target: &mut [u8] =
            unsafe { std::mem::transmute(&mut bytes.spare_capacity_mut()[..length]) };
        let total_read = input.read_exact(bytes_target).await?;
        assert_eq!(total_read, length);

        unsafe { bytes.set_len(length) };
    }
    Ok(Packet::decode(&bytes[..])?)
}

async fn write_packet(mut output: impl AsyncWrite + Unpin, packet: Packet) -> Result<()> {
    let encoded = packet.encode_to_vec();
    if encoded.len() as u64 > MAX_PACKET_LENGTH {
        bail!(
            "length of outbound packet is too high: {} > {MAX_PACKET_LENGTH}",
            encoded.len()
        );
    }
    output.write_u64(encoded.len() as u64).await?;
    output.write_all(&encoded[..]).await?;
    Ok(())
}

pub struct ProcessedPacket<'a> {
    pub command: CommandId,
    pub response: Option<ResponseHandle<'a>>,
    pub body: PacketBody,
    pub response_code: ResponseCode,
}

impl<'a> Into<ProcessedPacketOwned> for ProcessedPacket<'a> {
    fn into(self) -> ProcessedPacketOwned {
        ProcessedPacketOwned {
            command: self.command,
            response: self.response.map(Into::into),
            body: self.body,
            response_code: self.response_code,
        }
    }
}

pub struct ProcessedPacketOwned {
    pub command: CommandId,
    pub response: Option<ResponseHandleOwned>,
    pub body: PacketBody,
    pub response_code: ResponseCode,
}

fn process_inbound_packet<'a>(
    response: &'a mpsc::Sender<Packet>,
    packet: Packet,
) -> Result<ProcessedPacket<'a>, Packet> {
    let command = match CommandId::from_i32(packet.command) {
        Some(x) => x,
        None => {
            return Err(Packet {
                command: packet.command,
                id: packet.id,
                response: ResponseCode::ProtocolError as i32,
                expecting_response: false,
                packet_body: Some(PacketBody::ErrorMessage("unknown command_id".to_string())),
            });
        }
    };
    let response_code = match ResponseCode::from_i32(packet.response) {
        Some(x) => x,
        None => {
            return Err(Packet {
                command: packet.command,
                id: packet.id,
                response: ResponseCode::ProtocolError as i32,
                expecting_response: false,
                packet_body: Some(PacketBody::ErrorMessage(
                    "unknown response field".to_string(),
                )),
            });
        }
    };
    let response = if packet.expecting_response {
        Some(ResponseHandle {
            command,
            id: packet.id,
            sender: response,
        })
    } else {
        None
    };

    let body = match packet.packet_body {
        Some(x) => x,
        None => {
            return Err(Packet {
                command: packet.command,
                id: packet.id,
                response: ResponseCode::ProtocolError as i32,
                expecting_response: false,
                packet_body: Some(PacketBody::ErrorMessage("missing packet_body".to_string())),
            })
        }
    };
    Ok(ProcessedPacket {
        command,
        response,
        body,
        response_code,
    })
}

#[derive(Debug)]
pub enum RequestError {
    Closed,
    Dropped,
    Timeout,
}

impl Connection {
    pub async fn connect<A: ToSocketAddrs>(
        target: A,
        handler: impl RequestHandler,
    ) -> Result<Self> {
        let stream = TcpStream::connect(target).await?;
        let remote = stream.peer_addr()?;
        let (reader, writer) = stream.into_split();
        Ok(Self::accept(reader, writer, remote, handler))
    }

    pub async fn listen<R: RequestHandler>(
        bind: SocketAddr,
        handler: impl Fn(SocketAddr) -> R + Send + Sync + 'static,
        output: impl Fn(Self) + Send + Sync + 'static,
    ) -> Result<()> {
        let listener = TcpListener::bind(bind).await?;
        loop {
            let (stream, address) = listener.accept().await?;
            let handler = handler(address);
            let (reader, writer) = stream.into_split();
            output(Self::accept(reader, writer, address, handler));
        }
    }

    pub fn accept(
        mut reader: impl AsyncRead + Unpin + Send + Sync + 'static,
        mut writer: impl AsyncWrite + Unpin + Send + Sync + 'static,
        remote: SocketAddr,
        mut handler: impl RequestHandler,
    ) -> Self {
        let (inbound_sender, mut inbound_receiver) = mpsc::channel::<Packet>(CHANNEL_DEPTH);
        let (outbound_sender, mut outbound_receiver) = mpsc::channel::<Packet>(CHANNEL_DEPTH);

        tokio::spawn(async move {
            loop {
                let packet = match read_packet(&mut reader).await {
                    Err(e) => {
                        trace!("failed reading packet from remote {remote}: {e:?}");
                        break;
                    }
                    Ok(x) => x,
                };
                if inbound_sender.send(packet).await.is_err() {
                    break;
                }
            }
        });
        tokio::spawn(async move {
            while let Some(packet) = outbound_receiver.recv().await {
                if let Err(e) = write_packet(&mut writer, packet).await {
                    trace!("failed writing packet to remote {remote}: {e:?}");
                    break;
                }
            }
        });

        let pending_responses: Arc<DashMap<u64, oneshot::Sender<ProcessedPacketOwned>>> =
            Arc::new(DashMap::new());
        {
            let outbound_sender = outbound_sender.clone();
            let pending_responses = Arc::downgrade(&pending_responses);
            tokio::spawn(async move {
                while let Some(packet) = inbound_receiver.recv().await {
                    let id = packet.id;
                    let packet = match process_inbound_packet(&outbound_sender, packet) {
                        Ok(x) => x,
                        Err(packet) => {
                            outbound_sender.send(packet).await.ok();
                            continue;
                        }
                    };

                    if matches!(packet.response_code, ResponseCode::NotAResponse) {
                        if let Err(e) = handler.on_packet(packet).await {
                            error!("packet handler failed: {e:?}");
                        }
                        continue;
                    }
                    let pending_responses = match pending_responses.upgrade() {
                        Some(x) => x,
                        None => break,
                    };

                    if matches!(packet.response_code, ResponseCode::ProtocolError) {
                        warn!(
                            "protocol error from {remote}: {:?}",
                            packet.body.into_error_message()
                        );
                        pending_responses.remove(&id);
                        continue;
                    }

                    let pending_response = match pending_responses.remove(&id) {
                        None => {
                            warn!("received unexpected response for {id} from {remote}");
                            break;
                        }
                        Some(x) => x.1,
                    };
                    // we don't care if the receiver side dropped, they just didn't want the response
                    // i.e. i.e. it died or intentionally ignored it
                    pending_response.send(packet.into()).ok();
                }
                if let Err(e) = handler.on_disconnect().await {
                    error!("packet handler failed: {e:?}");
                }
            });
        }

        Self {
            next_local_id: AtomicU64::new(0),
            socket_addr: remote,
            outbound_channel: outbound_sender,
            pending_responses,
        }
    }

    /// Sends a request to the other end of this connection, returning `Ok(value)` when a response has been received.
    pub async fn request_with_response(
        &self,
        command: CommandId,
        body: PacketBody,
        timeout: Duration,
    ) -> Result<ProcessedPacketOwned, RequestError> {
        let timeout_end = Instant::now() + timeout;
        let id = self.next_local_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = oneshot::channel();
        self.pending_responses.insert(id, sender);

        let send_future = self.outbound_channel.send_timeout(
            Packet {
                command: command as i32,
                id,
                response: ResponseCode::NotAResponse as i32,
                expecting_response: true,
                packet_body: Some(body),
            },
            timeout,
        );

        // mechanism to avoid a memory leak in pending_responses from aborted futures
        struct SendFutureDropper<O, A: Future<Output = O>> {
            future: A,
            pending_responses: Option<Weak<DashMap<u64, oneshot::Sender<ProcessedPacketOwned>>>>,
            id: u64,
        }

        impl<O, A: Future<Output = O>> SendFutureDropper<O, A> {
            fn defuse(&mut self) {
                self.pending_responses.take();
            }
        }

        impl<O, A: Future<Output = O>> Drop for SendFutureDropper<O, A> {
            fn drop(&mut self) {
                if let Some(pending_responses) =
                    self.pending_responses.as_ref().and_then(|x| x.upgrade())
                {
                    pending_responses.remove(&self.id);
                }
            }
        }

        impl<O, A: Future<Output = O>> Future for SendFutureDropper<O, A> {
            type Output = O;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                // pin projection
                let future = unsafe { self.map_unchecked_mut(|s| &mut s.future) };
                future.poll(cx)
            }
        }

        let mut send_future = SendFutureDropper {
            future: send_future,
            pending_responses: Some(Arc::downgrade(&self.pending_responses)),
            id,
        };

        // SAFETY: we don't move send_future (and we ourselves are pinned by virtue of being async)
        let send_future_pinned = unsafe { Pin::new_unchecked(&mut send_future) };

        match send_future_pinned.await {
            Ok(()) => (),
            Err(SendTimeoutError::Timeout(_)) => return Err(RequestError::Timeout),
            Err(SendTimeoutError::Closed(_)) => return Err(RequestError::Closed),
        }
        send_future.defuse();
        match tokio::time::timeout_at(timeout_end.into(), receiver).await {
            Ok(Ok(packet)) => Ok(packet),
            Ok(Err(_)) => Err(RequestError::Dropped),
            Err(_) => Err(RequestError::Timeout),
        }
    }

    /// Returns `Ok` when the request has been sent. There is no application level acknowledgement, and the request may not have flushed out to network yet.
    pub async fn request(
        &self,
        command: CommandId,
        body: PacketBody,
        timeout: Duration,
    ) -> Result<(), RequestError> {
        let id = self.next_local_id.fetch_add(1, Ordering::Relaxed);
        match self
            .outbound_channel
            .send_timeout(
                Packet {
                    command: command as i32,
                    id,
                    response: ResponseCode::NotAResponse as i32,
                    expecting_response: false,
                    packet_body: Some(body),
                },
                timeout,
            )
            .await
        {
            Ok(()) => Ok(()),
            Err(SendTimeoutError::Timeout(_)) => Err(RequestError::Timeout),
            Err(SendTimeoutError::Closed(_)) => Err(RequestError::Closed),
        }
    }

    pub fn remote_addr(&self) -> SocketAddr {
        self.socket_addr
    }
}
