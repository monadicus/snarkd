pub use crate::peer_config::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Verbosity {
    None,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Log level verbosity, defaults to `info`
    pub verbosity: Verbosity,
    /// If not specified, an in-memory database is used
    pub database_file: Option<String>,
    /// At least this number of connections will be maintained
    pub minimum_connection_count: usize,
    /// No more than this number of connections will be maintained
    pub maximum_connection_count: usize,
    /// configuration for talking to trackers. defaults should be fine.
    pub tracker: PeerConfig,
    /// if true (default), then we announce our existence to the tracker
    pub enable_tracker_announce: bool,
    /// Seconds between peer syncs. Default 1.
    pub peer_sync_interval: usize,
    /// Port we are actually listening to
    pub listen_port: u16,
    /// Address that we are listening to. Defaults to 0.0.0.0
    pub listen_ip: Ipv4Addr,
    /// Port that we are receiving connections on. Generally the same as `listen_port` but a port rewrite firewall rule might change that.
    pub inbound_port: Option<u16>,
    /// Address that we are listening to for RPC. Defaults to 0.0.0.0
    pub rpc_ip: Ipv4Addr,
    /// Port that we are receiving RPC connections on, 0 for disabled
    pub rpc_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbosity: Verbosity::default(),
            database_file: None,
            minimum_connection_count: 20,
            maximum_connection_count: 50,
            tracker: PeerConfig::default(),
            enable_tracker_announce: true,
            peer_sync_interval: 1,
            listen_port: 5423,
            listen_ip: Ipv4Addr::UNSPECIFIED,
            inbound_port: None,
            rpc_ip: Ipv4Addr::UNSPECIFIED,
            rpc_port: 5422,
        }
    }
}

pub const CONFIG_ENV_VAR: &str = "SNARKD_CONFIG";
pub const CONFIG_NAME: &str = "snarkd.yaml";
pub const FULL_CONFIG_PATH: &str = "/etc/snarkd.yaml";
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static::lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = {
        let env_value = std::env::var(CONFIG_ENV_VAR).unwrap_or_default();
        if !env_value.trim().is_empty() {
            return Path::new(&*env_value).to_path_buf();
        }
        if Path::new(CONFIG_NAME).exists() {
            Path::new(CONFIG_NAME).to_path_buf()
        } else {
            Path::new(FULL_CONFIG_PATH).to_path_buf()
        }
    };
}

pub fn load_config() -> Result<Config> {
    if !CONFIG_PATH.exists() {
        return Err(anyhow!("cannot find config @ {}", CONFIG_PATH.display()));
    }

    let config_raw = std::fs::read_to_string(&*CONFIG_PATH)
        .map_err(|e| anyhow!("cannot read config @ {}: {e:?}", CONFIG_PATH.display()))?;

    let config: Config = serde_yaml::from_str(&config_raw)
        .map_err(|e| anyhow!("cannot parse config @ {}: {e:?}", CONFIG_PATH.display()))?;

    config
        .tracker
        .validate()
        .map_err(|e| anyhow!("invalid tracker config @ {}: {e:?}", CONFIG_PATH.display()))?;

    Ok(config)
}
