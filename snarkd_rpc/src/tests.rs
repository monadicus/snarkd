use jsonrpsee::{types::SubscriptionResult, SubscriptionSink};
use snarkd_storage::PeerData;

use crate::{
    client,
    common::{self, PeerMessage},
    server,
};

#[tokio::test]
/// Tests a fake impl of the snarkd rpc.
///
/// At the moment this looks identical to `test_websocket_transport` because
/// it's filled with boilerplate. When the actual RPC spec gets defined, this
/// will be much different
async fn test_snarkd_rpc() -> anyhow::Result<()> {
    use async_trait::async_trait;
    use common::{RpcClient, RpcError, RpcServer};

    struct TestServerImpl;
    #[async_trait]
    impl RpcServer for TestServerImpl {
        fn foo(&self) -> Result<String, RpcError> {
            Ok("foo".to_string())
        }

        async fn bar(&self, arg: String) -> Result<String, RpcError> {
            Ok(arg)
        }

        async fn list_peers(&self) -> Result<Vec<PeerData>, RpcError> {
            Ok(vec![])
        }

        fn subscribe_peers(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
            let _ = sink.send(&PeerMessage::Connect("0.0.0.0:0".parse().unwrap()));
            Ok(())
        }
    }

    let (addr, server) =
        server::websocket_server(TestServerImpl.into_rpc(), "127.0.0.1:0".parse()?).await?;
    let rpc = client::websocket_client(format!("ws://{addr}").parse()?).await?;

    assert_eq!(rpc.foo().await?, "foo");
    assert_eq!(rpc.bar("bar".to_string()).await?, "bar");
    assert_eq!(rpc.list_peers().await?.len(), 0);
    let mut subscription = rpc.subscribe_peers().await?;
    assert!(subscription.next().await.is_some());
    subscription.unsubscribe().await?;

    server.stop()?;

    Ok(())
}

#[tokio::test]
/// Creates a dummy test rpc separate from the snarkd rpc impl for testing
/// the websocket transport
async fn test_websocket_transport() -> anyhow::Result<()> {
    use async_trait::async_trait;
    use common::RpcError;
    use jsonrpsee::proc_macros::rpc;

    #[rpc(server, client, namespace = "test")]
    #[async_trait]
    pub trait Test {
        #[method(name = "foo")]
        fn foo(&self) -> Result<String, RpcError>;

        #[method(name = "bar")]
        async fn bar(&self, arg: String) -> Result<String, RpcError>;
    }

    struct TestServerImpl;
    #[async_trait]
    impl TestServer for TestServerImpl {
        fn foo(&self) -> Result<String, RpcError> {
            Ok("foo".to_string())
        }

        async fn bar(&self, arg: String) -> Result<String, RpcError> {
            Ok(arg)
        }
    }

    let (addr, server) =
        server::websocket_server(TestServerImpl.into_rpc(), "127.0.0.1:0".parse()?).await?;
    let rpc = client::websocket_client(format!("ws://{addr}").parse()?).await?;

    assert_eq!(rpc.foo().await?, "foo");
    assert_eq!(rpc.bar("bar".to_string()).await?, "bar");

    server.stop()?;

    Ok(())
}
