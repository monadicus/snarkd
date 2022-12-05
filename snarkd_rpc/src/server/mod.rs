use anyhow::Result;
use jsonrpsee::ws_server::{self, WsServerHandle};
pub use jsonrpsee::RpcModule;
use std::net::SocketAddr;

/// Serves this RpcServer via websocket
pub async fn websocket_server<C>(
    module: RpcModule<C>,
    addr: SocketAddr,
) -> Result<(SocketAddr, WsServerHandle)> {
    // in the future, we can replace this default with settings
    let server = ws_server::WsServerBuilder::default().build(addr).await?;
    let addr = server.local_addr()?;
    let handle = server.start(module)?;

    // this handle can call handle.stop() to stop
    Ok((addr, handle))
}
