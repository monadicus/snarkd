use std::sync::{Arc, RwLock};

use anyhow::Result;
pub use jsonrpsee::{http_server, ws_server, RpcModule};

pub trait RPCServer: Send + Sync + Sized {
    /// Returns foo
    fn foo(&self) -> String;

    fn module(&'static mut self) -> Result<RpcModule<Arc<RwLock<&'static mut Self>>>> {
        let mut module = RpcModule::new(Arc::new(RwLock::new(self)));

        // calls foo
        module.register_method("foo", |_params, ctx| {
            Ok(ctx.read().map_err(|e| anyhow::anyhow!("{e:?}"))?.foo())
        })?;

        Ok(module)
    }
}
