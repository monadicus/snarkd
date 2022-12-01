use chrono::{DateTime, Utc};
pub use jsonrpsee::core::Error as RpcError;
use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
use snarkd_common::config::Config;
pub use snarkd_storage::PeerData;
use std::{collections::HashMap, net::SocketAddr};
use uuid::Uuid;

#[rpc(server, client, namespace = "snarkd")]
#[async_trait]
pub trait Rpc {
    #[method(name = "foo")]
    /// Returns a string
    fn foo(&self) -> Result<String, RpcError>;

    #[method(name = "bar")]
    /// Returns a future, accepts an argument
    async fn bar(&self, arg: String) -> Result<String, RpcError>;

    #[method(name = "metadata")]
    /// Fetches current node metadata
    fn metadata(&self) -> Result<NodeMetadata, RpcError>;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// generated uuid of the node
    pub node_id: Uuid,
    /// snarkd version
    pub version: String,
    /// config at node create time
    pub config: Config,
    /// location of config
    pub config_path: String,
    /// Working directory of snarkd
    pub cwd: String,
    /// Node start time in seconds since unix epoch
    pub start_time: DateTime<Utc>,
}
