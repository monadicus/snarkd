use std::{net::SocketAddr, sync::Arc};

use anyhow::{Result, bail};
use chrono::Utc;
use log::{debug, info};
use snarkd_common::Digest;
use snarkd_consensus::Consensus;
use snarkd_network::{
    proto::{packet::PacketBody, Block, Introduction, Ping, ResponseCode, Transaction, Blocks},
    RequestHandler, ResponseHandle,
};
use snarkd_storage::Database;
use tokio::sync::{oneshot, RwLock};

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
        _response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        let mut consensus = self.peer_book.consensus.write().await;
        for block in blocks {
            consensus.receive_block(block.try_into()?).await?;
        }
        Ok(())
    }

    async fn on_transactions(
        &mut self,
        transactions: Vec<Transaction>,
        _response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        let mut consensus = self.peer_book.consensus.write().await;
        for transaction in transactions {
            // consensus.receive_transaction(transaction).await;
        }
        Ok(())
    }

    async fn on_get_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> Result<()> {
        let Some(response) = response else {
            bail!("not expecting response for get_blocks");
        };
        let mut out = Vec::with_capacity(digests.len());
        for digest in digests {
            if let Some(block) = self.peer_book.database.get_block(digest).await? {
                out.push(snarkd_network::proto::Block {
                    header: Some(snarkd_network::proto::BlockHeader {
                        canon_height: block.header.height,
                        hash: Some(block.header.block_hash),
                        previous_hash: Some(block.header.previous_hash),
                        nonce: block.header.nonce,
                        network: block.header.network as u32,
                        coinbase_target: block.header.coinbase_target,
                        timestamp: block.header.timestamp as u64,
                    }),
                    transactions: vec![]
                    // transactions: block.transactions.into_iter().map(|transaction| snarkd_network::proto::Transaction {
                        
                    // }),
                });
            }
        }
        response.send(ResponseCode::Ok, PacketBody::Blocks(Blocks { blocks: out })).await;
        Ok(())
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
