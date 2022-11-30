pub use jsonrpsee::core::Error as RpcError;
use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
pub use snarkd_storage::PeerData;
use std::{collections::HashMap, net::SocketAddr};

#[rpc(server, client, namespace = "snarkd")]
#[async_trait]
pub trait Rpc {
    #[method(name = "foo")]
    /// Returns a string
    fn foo(&self) -> Result<String, RpcError>;

    #[method(name = "bar")]
    /// Returns a future, accepts an argument
    async fn bar(&self, arg: String) -> Result<String, RpcError>;

    #[method(name = "get_peers")]
    /// Returns a list of peer data
    async fn get_peers(&self) -> Result<HashMap<SocketAddr, PeerData>, RpcError>;

    #[subscription(name = "subscribe_peers", item = PeerMessage)]
    /// Subscription that produces a PeerMessage.
    fn subscribe_peers(&self);
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PeerMessage {
    Attempt(SocketAddr),
    Accept(SocketAddr),
    Connect { address: SocketAddr, peer: PeerData },
    Handshake { address: SocketAddr, peer: PeerData },
    Update { address: SocketAddr, peer: PeerData },
    Disconnect(SocketAddr),
}
