use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
pub use snarkd_rpc::common::PeerMessage;
use snarkd_rpc::{
    common::{RpcError, RpcServer},
    jsonrpsee::{core::error::SubscriptionClosed, types::SubscriptionResult, SubscriptionSink},
    server::RpcModule,
};
use snarkd_storage::PeerData;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::BroadcastStream;

use crate::peer_book::PeerBook;

pub struct RpcChannels {
    peer_broadcast: Sender<PeerMessage>,
}

impl RpcChannels {
    pub fn new() -> Self {
        Self {
            peer_broadcast: tokio::sync::broadcast::channel(16).0,
        }
    }

    pub fn peer_message(&self, msg: PeerMessage) {
        if let Err(e) = self.peer_broadcast.send(msg) {
            debug!("failed to broadcast rpc peer message: {}", e.to_string());
        }
    }
}

pub struct SnarkdRpc {
    pub peer_book: PeerBook,
    pub channels: Arc<RpcChannels>,
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

    fn subscribe_peers(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
        let rx = BroadcastStream::new(self.channels.peer_broadcast.clone().subscribe());

        tokio::spawn(async move {
            match sink.pipe_from_try_stream(rx).await {
                SubscriptionClosed::Success => {
                    sink.close(SubscriptionClosed::Success);
                }
                SubscriptionClosed::RemotePeerAborted => (),
                SubscriptionClosed::Failed(err) => {
                    sink.close(err);
                }
            };
        });
        Ok(())
    }
}

impl SnarkdRpc {
    pub(crate) fn module(self) -> RpcModule<SnarkdRpc> {
        self.into_rpc()
    }
}
