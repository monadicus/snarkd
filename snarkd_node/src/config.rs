use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use snarkd_peer::config::PeerConfig;

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

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    /// Log level verbosity, defaults to `info`
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
    /// Port that we are receiving connections on. Generally the same as `listen_port` but a port rewrite firewall rule might change that.
    #[serde(default = "default_u16::<5423>")]
    pub inbound_port: u16,
}

const CONFIG_ENV_VAR: &str = "SNARKD_CONFIG";
const CONFIG_NAME: &str = "snarkd.yaml";
const FULL_CONFIG_PATH: &str = "/etc/snarkd.yaml";

lazy_static::lazy_static! {
    static ref CONFIG_PATH: PathBuf = {
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
    // ArcSwap to allow hotloading later on
    pub static ref CONFIG: ArcSwap<Config> = {
        println!("loading config @ {}", CONFIG_PATH.display());
        if !CONFIG_PATH.exists() {
            eprintln!("cannot find config @ {}", CONFIG_PATH.display());
            std::process::exit(1);
        }
        let config_raw = match std::fs::read_to_string(&*CONFIG_PATH) {
            Err(e) => {
                eprintln!("cannot read config @ {}: {e:?}", CONFIG_PATH.display());
                std::process::exit(1);
            },
            Ok(x) => x,
        };
        match serde_yaml::from_str(&*config_raw) {
            Ok(x) => ArcSwap::new(Arc::new(x)),
            Err(e) => {
                eprintln!("cannot parse config @ {}: {e:?}", CONFIG_PATH.display());
                std::process::exit(1);
            }
        }
    };
}
