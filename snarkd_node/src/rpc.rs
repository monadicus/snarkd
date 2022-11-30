use std::{collections::HashMap, net::SocketAddr, sync::Arc};

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

pub enum RpcChannels {
    Disabled,
    Enabled { peer_broadcast: Sender<PeerMessage> },
}

impl RpcChannels {
    pub fn new(enabled: bool) -> Self {
        if enabled {
            Self::Enabled {
                peer_broadcast: tokio::sync::broadcast::channel(16).0,
            }
        } else {
            Self::Disabled
        }
    }

    pub fn peer_message(&self, msg: PeerMessage) {
        if let Self::Enabled { peer_broadcast, .. } = self {
            if let Err(e) = peer_broadcast.send(msg) {
                debug!("failed to broadcast rpc peer message: {}", e.to_string());
            }
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

    async fn get_peers(&self) -> Result<HashMap<SocketAddr, PeerData>, RpcError> {
        Ok(self
            .peer_book
            .connected_peers()
            .map(|kv| (*kv.key(), kv.value().data))
            .collect())
    }

    fn subscribe_peers(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
        let channel = match &*self.channels {
            RpcChannels::Enabled { peer_broadcast, .. } => peer_broadcast,
            _ => unreachable!("rpc server was provided disabled channels"),
        };

        let rx = BroadcastStream::new(channel.clone().subscribe());

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
