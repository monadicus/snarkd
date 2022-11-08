use anyhow::Result;
pub use jsonrpsee::core::Error as RpcError;
use jsonrpsee::proc_macros::rpc;

#[rpc(server, client, namespace = "snarkd")]
#[async_trait]
pub trait Rpc {
    #[method(name = "foo")]
    /// Returns a string
    fn foo(&self) -> Result<String, RpcError>;

    #[method(name = "bar")]
    /// Returns a future, accepts an argument
    async fn bar(&self, arg: String) -> Result<String, RpcError>;
}
