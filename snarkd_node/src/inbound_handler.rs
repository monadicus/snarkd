use std::net::SocketAddr;

use snarkd_common::Digest;
use snarkd_network::{
    proto::{Block, Introduction, Transaction},
    RequestHandler, ResponseHandle,
};

pub struct InboundHandler {
    pub address: SocketAddr,
}

#[async_trait::async_trait]
impl RequestHandler for InboundHandler {
    async fn on_introduction(
        &mut self,
        introduction: Introduction,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_blocks(
        &mut self,
        blocks: Vec<Block>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transactions(
        &mut self,
        transactions: Vec<Transaction>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_get_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_sync_memory_pool(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_sync_peers(
        &mut self,
        peers: Vec<String>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_sync_blocks(
        &mut self,
        digests: Vec<Digest>,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_ping(
        &mut self,
        timestamp: u64,
        response: Option<ResponseHandle<'_>>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
