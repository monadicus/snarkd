use std::{net::SocketAddr, sync::Arc, time::Duration};

use crate::config::{CONFIG, NODE_ID};
use crate::rpc::RpcChannels;
use crate::{inbound_handler::InboundHandler, peer_book::PeerBook};
use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use snarkd_common::config::VERSION;
use snarkd_network::{
    proto::{packet::PacketBody, CommandId, Introduction, ResponseCode},
    Connection,
};
use snarkd_rpc::common::PeerMessage;
use snarkd_storage::{Database, PeerData, PeerDirection};
use tokio::task::JoinHandle;

enum ConnectionState {
    Connected(Arc<Connection>),
    Connecting(JoinHandle<()>),
    Disconnected,
}

pub struct Peer {
    pub address: SocketAddr,
    connection: ConnectionState,
    pub data: PeerData,
    // if true, we have not saved state to disk yet
    pub dirty: bool,
    recent_failures: Vec<DateTime<Utc>>,
    rpc_channels: Arc<RpcChannels>,
}

const FAILURE_EXPIRY_TIME: Duration = Duration::from_secs(15 * 60);
const FAILURE_THRESHOLD: usize = 5;
pub const MAX_PEER_INACTIVITY: Duration = Duration::from_secs(30);
pub const PEER_TIMEOUT: Duration = Duration::from_secs(5);
pub const PEER_PING_INTERVAL: Duration = Duration::from_secs(10);

pub fn form_introduction(address: SocketAddr) -> PacketBody {
    let config = CONFIG.load();
    PacketBody::Introduction(Introduction {
        target_address: address.to_string(),
        version: VERSION.to_string(),
        instance_id: NODE_ID.as_bytes().to_vec(),
        inbound_port: config.inbound_port.unwrap_or(config.listen_port) as u32,
    })
}

impl Peer {
    pub fn new(address: SocketAddr, data: PeerData, rpc_channels: Arc<RpcChannels>) -> Self {
        Self {
            address,
            connection: ConnectionState::Disconnected,
            data,
            recent_failures: vec![],
            dirty: false,
            rpc_channels,
        }
    }

    pub async fn save(&mut self, db: &Database) -> Result<()> {
        self.data.save(db).await?;
        self.dirty = false;
        Ok(())
    }

    pub fn recent_failures(&mut self) -> usize {
        let now = Utc::now();
        if self.recent_failures.len() >= FAILURE_THRESHOLD {
            self.recent_failures.retain(|x| {
                now.signed_duration_since(*x)
                    < chrono::Duration::from_std(FAILURE_EXPIRY_TIME).unwrap()
            });
        }
        self.recent_failures.len()
    }

    pub fn is_inactive(&self) -> bool {
        if !self.is_connected() {
            return false;
        }
        let last_seen = self.data.last_seen;
        if let Some(last_seen) = last_seen {
            Utc::now() - last_seen > chrono::Duration::from_std(MAX_PEER_INACTIVITY).unwrap()
        } else {
            false
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.connection, ConnectionState::Connected(_))
    }

    pub fn disconnect(&mut self) {
        if matches!(self.connection, ConnectionState::Disconnected) {
            return;
        }
        info!("disconnecting from {}", self.address);
        self.connection = ConnectionState::Disconnected;
        self.rpc_channels
            .peer_message(PeerMessage::Disconnect(self.address))
    }

    pub fn connect(
        &mut self,
        peer_book: PeerBook,
        output: impl FnOnce(Option<Connection>) + Send + Sync + 'static,
    ) {
        let address = self.address;
        let handle = tokio::spawn(async move {
            match Connection::connect(address, InboundHandler::new(address, peer_book, None)).await
            {
                Ok(connection) => output(Some(connection)),
                Err(e) => {
                    debug!("failed to connect to peer {address}: {e:?}");
                    output(None)
                }
            }
        });
        if !matches!(self.connection, ConnectionState::Connecting(_)) {
            self.rpc_channels
                .peer_message(PeerMessage::Attempt(address));
        }
        self.connection = ConnectionState::Connecting(handle);
    }

    pub fn register_failed_connection(&mut self) {
        self.data.connection_fail_count += 1;
        self.dirty = true;
    }

    pub fn register_connection(&mut self, direction: PeerDirection, connection: Connection) {
        let connection = Arc::new(connection);
        if matches!(direction, PeerDirection::Outbound) {
            let connection = connection.clone();
            let address = self.address;
            tokio::spawn(async move {
                if let Err(e) = connection
                    .request(
                        CommandId::Introduction,
                        form_introduction(address),
                        PEER_TIMEOUT,
                    )
                    .await
                {
                    error!("failed to send introduction to peer: {e:?}");
                }
            });
        }
        self.connection = ConnectionState::Connected(connection);
        self.data.last_peer_direction = direction;
        self.data.connection_success_count += 1;
        self.data.last_connected = Some(Utc::now());
        self.data.last_seen = self.data.last_connected;
        self.dirty = true;

        self.rpc_channels.peer_message(PeerMessage::Connect {
            address: self.address,
            peer: self.data,
        });
    }

    pub fn judge_bad(&mut self) -> bool {
        // warn!("Peer {} has a low quality score; disconnecting.", self.address);
        self.recent_failures() >= FAILURE_THRESHOLD || self.is_inactive()
    }

    pub fn connection(&self) -> Option<&Arc<Connection>> {
        match &self.connection {
            ConnectionState::Connected(c) => Some(c),
            _ => None,
        }
    }

    pub fn fail(&mut self) {
        self.recent_failures.push(Utc::now());
    }

    pub fn start_ping(&self, peer_book: PeerBook) {
        let connection = match self.connection() {
            Some(x) => x.clone(),
            None => return,
        };
        let address = self.address;
        tokio::spawn(async move {
            let response = connection
                .request_with_response(
                    CommandId::Ping,
                    PacketBody::PingPong(snarkd_network::proto::Ping {
                        block_height: 0, // todo
                        timestamp: Utc::now().timestamp() as u64,
                    }),
                    PEER_TIMEOUT,
                )
                .await;
            let mut self_ = match peer_book.peer_mut(&address) {
                None => return,
                Some(x) => x,
            };
            match response {
                Ok(pong) => {
                    self_.data.last_seen = Some(Utc::now());
                    if !matches!(pong.response_code, ResponseCode::Ok) {
                        self_.fail();
                        self_.disconnect();
                    }
                    if let Some(pong) = pong.body.into_ping_pong() {
                        self_.data.block_height = pong.block_height;
                    } else {
                        self_.fail();
                        self_.disconnect();
                    }
                    debug!("outbound ping complete for {address}");
                    self_.dirty = true;
                }
                Err(_) => {
                    self_.disconnect();
                }
            }
        });
    }
}
