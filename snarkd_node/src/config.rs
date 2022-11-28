use arc_swap::ArcSwap;
use std::sync::Arc;
use uuid::Uuid;

use snarkd_common::config::{load_config, Config, CONFIG_PATH};

lazy_static::lazy_static! {

    // ArcSwap to allow hotloading later on
    pub static ref CONFIG: ArcSwap<Config> = {
        println!("loading config @ {}", CONFIG_PATH.display());
        match load_config() {
            Err(e) => {
                eprintln!("{e:?}");
                std::process::exit(1);
            },
            Ok(conf) => ArcSwap::new(Arc::new(conf))
        }
    };
    /// unique node id, used to avoid cyclic connections
    pub static ref NODE_ID: Uuid = Uuid::new_v4();
}
