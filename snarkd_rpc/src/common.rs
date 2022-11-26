pub use jsonrpsee::core::Error as RpcError;
use jsonrpsee::proc_macros::rpc;
pub use snarkd_storage::PeerData;

#[rpc(server, client, namespace = "snarkd")]
#[async_trait]
pub trait Rpc {
    #[method(name = "foo")]
    /// Returns a string
    fn foo(&self) -> Result<String, RpcError>;

    #[method(name = "bar")]
    /// Returns a future, accepts an argument
    async fn bar(&self, arg: String) -> Result<String, RpcError>;

    #[method(name = "list_peers")]
    /// Returns a list of peer data
    async fn list_peers(&self) -> Result<Vec<PeerData>, RpcError>;
}
