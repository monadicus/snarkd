use anyhow::Result;
use snarkd_rpc::{
    client::{websocket_client, Client},
    common::RpcClient,
};
use std::net::SocketAddrV4;

pub struct SnarkdClient {
    rpc: Client,
}

impl SnarkdClient {
    pub async fn new(addr: SocketAddrV4) -> Result<Self> {
        let rpc = websocket_client(format!("ws://{addr}").parse()?).await?;
        Ok(SnarkdClient { rpc })
    }

    pub async fn foo(&self) -> Result<String> {
        Ok(self.rpc.foo().await?)
    }

    pub async fn bar(&self, arg: String) -> Result<String> {
        Ok(self.rpc.bar(arg).await?)
    }
}
