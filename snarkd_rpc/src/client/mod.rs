use anyhow::Result;
use jsonrpsee::{
    core::client::{Client, ClientBuilder, ClientT, TransportReceiverT, TransportSenderT},
    rpc_params,
};

pub struct RPCClient {
    client: Client,
}

impl RPCClient {
    pub fn new<S, R>(sender: S, receiver: R) -> Result<Self>
    where
        S: TransportSenderT + Send,
        R: TransportReceiverT + Send,
    {
        let client = ClientBuilder::default().build_with_tokio(sender, receiver);
        Ok(RPCClient { client })
    }

    /// Requests foo
    pub async fn foo(&self) -> Result<String> {
        Ok(self.client.request("foo", rpc_params![]).await?)
    }
}
