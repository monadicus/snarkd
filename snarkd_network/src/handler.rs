use crate::{
    proto::{packet::PacketBody, *},
    ProcessedPacket,
};
use anyhow::{Context, Result};
use log::debug;
use snarkd_common::Digest;
use tokio::sync::mpsc;

pub struct ResponseHandle<'a> {
    pub(crate) command: CommandId,
    pub(crate) id: u64,
    pub(crate) sender: &'a mpsc::Sender<Packet>,
}

pub struct ResponseHandleOwned {
    command: CommandId,
    id: u64,
    sender: mpsc::Sender<Packet>,
}

impl<'a> ResponseHandle<'a> {
    pub async fn send(self, code: ResponseCode, response: PacketBody) {
        assert!(!matches!(code, ResponseCode::NotAResponse));
        let out = self
            .sender
            .send(Packet {
                command: self.command as i32,
                id: self.id,
                response: code as i32,
                expecting_response: false,
                packet_body: Some(response),
            })
            .await;
        if out.is_err() {
            debug!("response failed, remote was gone before sending");
        }
    }
}

impl<'a> Into<ResponseHandleOwned> for ResponseHandle<'a> {
    fn into(self) -> ResponseHandleOwned {
        ResponseHandleOwned {
            command: self.command,
            id: self.id,
            sender: self.sender.clone(),
        }
    }
}

impl ResponseHandleOwned {
    pub async fn send(self, code: ResponseCode, response: PacketBody) {
        ResponseHandle {
            command: self.command,
            id: self.id,
            sender: &self.sender,
        }
        .send(code, response)
        .await;
    }
}

// This handles inbound request packets, this can block receiving future packets so should defer quickly.
// All errors result in an error log and connection closure.
#[async_trait::async_trait]
pub trait RequestHandler {
    async fn on_introduction(
        &mut self,
        introduction: Introduction,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_blocks(
        &mut self,
        blocks: Vec<Block>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_transactions(
        &mut self,
        transactions: Vec<Transaction>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_get_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    // todo: can we use a bloom filter for memory pool?
    async fn on_sync_memory_pool(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_sync_peers(
        &mut self,
        peers: Vec<String>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_sync_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()>;

    async fn on_ping(&mut self, timestamp: u64, response: Option<ResponseHandle<'_>>)
        -> Result<()>;

    async fn on_packet(&mut self, packet: ProcessedPacket<'_>) -> Result<()> {
        assert!(matches!(packet.response_code, ResponseCode::NotAResponse));
        match packet.command {
            CommandId::Introduction => {
                let introduction = packet
                    .body
                    .into_introduction()
                    .context("invalid packet body value")?;
                self.on_introduction(introduction, packet.response).await?;
            }
            CommandId::BlockTransmission => {
                let blocks = packet
                    .body
                    .into_blocks()
                    .context("invalid packet body value")?;
                self.on_blocks(blocks.blocks, packet.response).await?;
            }
            CommandId::TransactionTransmission => {
                let transactions = packet
                    .body
                    .into_transactions()
                    .context("invalid packet body value")?;
                self.on_transactions(transactions.transactions, packet.response)
                    .await?;
            }
            CommandId::GetBlocks => {
                let digests = packet
                    .body
                    .into_digests()
                    .context("invalid packet body value")?;
                self.on_get_blocks(digests, packet.response).await?;
            }
            CommandId::SyncMemoryPool => {
                let digests = packet
                    .body
                    .into_digests()
                    .context("invalid packet body value")?;
                self.on_sync_memory_pool(digests, packet.response).await?;
            }
            CommandId::SyncPeers => {
                let peer_list = packet
                    .body
                    .into_peer_list()
                    .context("invalid packet body value")?;
                self.on_sync_peers(peer_list.peers, packet.response).await?;
            }
            CommandId::SyncBlocks => {
                let digests = packet
                    .body
                    .into_digests()
                    .context("invalid packet body value")?;
                self.on_sync_blocks(digests, packet.response).await?;
            }
            CommandId::Ping => {
                let ping = packet
                    .body
                    .into_ping_pong()
                    .context("invalid packet body value")?;
                self.on_ping(ping, packet.response).await?;
            }
        }
        Ok(())
    }
}
