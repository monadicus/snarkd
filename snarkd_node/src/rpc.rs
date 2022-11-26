use async_trait::async_trait;
use snarkd_rpc::{
    common::{RpcError, RpcServer},
    server::RpcModule,
};
use snarkd_storage::PeerData;

use crate::peer_book::PeerBook;

pub struct SnarkdRpc {
    pub peer_book: PeerBook,
}

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

    async fn list_peers(&self) -> Result<Vec<PeerData>, RpcError> {
        Ok(self
            .peer_book
            .connected_peers()
            .map(|kv| kv.value().data)
            .collect())
    }
}

impl SnarkdRpc {
    pub(crate) fn module(self) -> RpcModule<SnarkdRpc> {
        self.into_rpc()
    }
}
