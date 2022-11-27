use anyhow::{anyhow, Result};
use snarkd_rpc::{
    client::{websocket_client, Client},
    common::{PeerData, PeerMessage, RpcClient, RpcError},
    jsonrpsee::core::client::Subscription,
};
use url::Url;

pub struct SnarkdClient {
    url: Url,
    pub rpc: Client,
}

impl SnarkdClient {
    pub async fn new(url: Url) -> Result<Self> {
        let rpc = match url.scheme() {
            "ws" | "wss" => websocket_client(url.to_string().parse()?).await?,
            scheme => return Err(anyhow!("Unsupported client scheme {scheme}")),
        };
        Ok(SnarkdClient { rpc, url })
    }

    pub fn get_url(&self) -> Url {
        self.url.clone()
    }

    pub async fn foo(&self) -> Result<String, RpcError> {
        self.rpc.foo().await
    }

    pub async fn bar(&self, arg: String) -> Result<String, RpcError> {
        self.rpc.bar(arg).await
    }

    pub async fn list_peers(&self) -> Result<Vec<PeerData>, RpcError> {
        self.rpc.list_peers().await
    }

    pub async fn subscribe_peers(&self) -> Result<Subscription<PeerMessage>, RpcError> {
        self.rpc.subscribe_peers().await
    }
}
