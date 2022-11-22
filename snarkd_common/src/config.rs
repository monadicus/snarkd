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

fn default_usize<const D: usize>() -> usize {
    D
}

fn default_u16<const D: u16>() -> u16 {
    D
}

fn default_bool<const D: bool>() -> bool {
    D
}

fn default_listen_ip() -> Ipv4Addr {
    Ipv4Addr::UNSPECIFIED
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Log level verbosity, defaults to `info`
    #[serde(default)]
    pub verbosity: Verbosity,
    /// If not specified, an in-memory database is used
    pub database_file: Option<String>,
    /// At least this number of connections will be maintained
    #[serde(default = "default_usize::<20>")]
    pub minimum_connection_count: usize,
    /// No more than this number of connections will be maintained
    #[serde(default = "default_usize::<50>")]
    pub maximum_connection_count: usize,
    /// configuration for talking to trackers. defaults should be fine.
    #[serde(default)]
    pub tracker: PeerConfig,
    /// if true (default), then we announce our existence to the tracker
    #[serde(default = "default_bool::<true>")]
    pub enable_tracker_announce: bool,
    /// Seconds between peer syncs. Default 1.
    #[serde(default = "default_usize::<1>")]
    pub peer_sync_interval: usize,
    /// Port we are actually listening to
    #[serde(default = "default_u16::<5423>")]
    pub listen_port: u16,
    /// Address that we are listening to. Defaults to 0.0.0.0
    #[serde(default = "default_listen_ip")]
    pub listen_ip: Ipv4Addr,
    /// Port that we are receiving connections on. Generally the same as `listen_port` but a port rewrite firewall rule might change that.
    pub inbound_port: Option<u16>,
    /// Address that we are listening to for RPC. Defaults to 0.0.0.0
    #[serde(default = "default_listen_ip")]
    pub rpc_ip: Ipv4Addr,
    /// Port that we are receiving RPC connections on, 0 for disabled
    #[serde(default = "default_u16::<5422>")]
    pub rpc_port: u16,
}

pub const CONFIG_ENV_VAR: &str = "SNARKD_CONFIG";
pub const CONFIG_NAME: &str = "snarkd.yaml";
pub const FULL_CONFIG_PATH: &str = "/etc/snarkd.yaml";
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn load_config() -> Result<Config> {
    if !CONFIG_PATH.exists() {
        return Err(anyhow!("cannot find config @ {}", CONFIG_PATH.display()));
    }

    let config_raw = std::fs::read_to_string(&*CONFIG_PATH)
        .map_err(|e| anyhow!("cannot read config @ {}: {e:?}", CONFIG_PATH.display()))?;

    serde_yaml::from_str(&config_raw)
        .map_err(|e| anyhow!("cannot parse config @ {}: {e:?}", CONFIG_PATH.display()))
}

impl Default for Config {
    fn default() -> Self {
        serde_yaml::from_str("{}").unwrap()
    }
}

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
