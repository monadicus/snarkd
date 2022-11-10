pub use crate::common::RpcClient;
use anyhow::{Ok, Result};
pub use jsonrpsee::core::client::Client;
use jsonrpsee::{
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::client::{ClientBuilder, TransportReceiverT, TransportSenderT},
};

/// Creates an RPC client given transport a sender and receiver.
pub fn new_client<S, R>(sender: S, receiver: R) -> Client
where
    S: TransportSenderT + Send,
    R: TransportReceiverT + Send,
{
    // may want to add settings for this when we know more about the data we're sending
    ClientBuilder::default().build_with_tokio(sender, receiver)
}

/// Creates a websocket client
/// - `uri`: Websocket uri, like `ws://127.0.0.1:8080`
pub async fn websocket_client(uri: Uri) -> Result<Client> {
    let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
    Ok(new_client(tx, rx))
}
