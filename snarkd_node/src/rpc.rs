use async_trait::async_trait;
use snarkd_rpc::{
    common::{RpcError, RpcServer},
    server::RpcModule,
};

pub struct SnarkdRpc;

#[async_trait]
impl RpcServer for SnarkdRpc {
    fn foo(&self) -> Result<String, RpcError> {
        Ok("foo".to_string())
    }

    async fn bar(&self, arg: String) -> Result<String, RpcError> {
        if arg == "foo" {
            Err(RpcError::Custom("bad input foo".to_string()))
        } else {
            Ok(arg)
        }
    }
}

pub(crate) fn module() -> RpcModule<SnarkdRpc> {
    SnarkdRpc.into_rpc()
}
