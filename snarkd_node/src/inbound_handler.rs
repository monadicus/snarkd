use std::net::SocketAddr;

use anyhow::Result;
use chrono::Utc;
use log::{debug, info};
use snarkd_common::Digest;
use snarkd_network::{
    proto::{packet::PacketBody, Block, Introduction, Ping, ResponseCode, Transaction},
    RequestHandler, ResponseHandle,
};
use tokio::sync::oneshot;

use crate::peer_book::PeerBook;

pub struct InboundHandler {
    peer_book: PeerBook,
    address: SocketAddr,
    intro_sender: Option<oneshot::Sender<Introduction>>,
}

impl InboundHandler {
    pub fn new(
        address: SocketAddr,
        peer_book: PeerBook,
        intro_sender: Option<oneshot::Sender<Introduction>>,
    ) -> Self {
        Self {
            address,
            intro_sender,
            peer_book,
        }
    }
}

#[async_trait::async_trait]
impl RequestHandler for InboundHandler {
    async fn on_introduction(
        &mut self,
        introduction: Introduction,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        let intro_sender = match self.intro_sender.take() {
            Some(x) => x,
            None => return Ok(()),
        };
        self.address.set_port(introduction.inbound_port as u16);
        info!("introduction received from {}", self.address);
        intro_sender.send(introduction).ok();
        if let Some(response) = response {
            response
                .send(
                    ResponseCode::Ok,
                    crate::peer::form_introduction(self.address),
                )
                .await;
        }
        Ok(())
    }

    async fn on_blocks(
        &mut self,
        blocks: Vec<Block>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_transactions(
        &mut self,
        transactions: Vec<Transaction>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_get_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_sync_memory_pool(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_sync_peers(
        &mut self,
        peers: Vec<String>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_sync_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        todo!()
    }

    async fn on_ping(&mut self, ping: Ping, response: Option<ResponseHandle<'_>>) -> Result<()> {
        if let Some(mut peer) = self.peer_book.peer_mut(&self.address) {
            peer.data.block_height = ping.block_height;
            peer.data.last_seen = Some(Utc::now());
            peer.dirty = true;
        }

        if let Some(response) = response {
            response
                .send(
                    ResponseCode::Ok,
                    PacketBody::PingPong(Ping {
                        timestamp: ping.timestamp,
                        block_height: 0, //todo
                    }),
                )
                .await;
        }
        debug!("inbound ping complete for {}", self.address);
        Ok(())
    }

    async fn on_disconnect(&mut self) -> Result<()> {
        if let Some(mut peer) = self.peer_book.peer_mut(&self.address) {
            peer.disconnect();
        }
        Ok(())
    }
}
