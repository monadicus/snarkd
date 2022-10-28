use clap::Parser;
use config::{Verbosity, CONFIG};
use log::{error, warn, LevelFilter};
use snarkd_peer::{config::PeerConfig, tracker::test_http_client};
use snarkd_storage::Database;

mod config;

/// Snarkd Blockchain Node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() {
    lazy_static::initialize(&CONFIG);

    let config = CONFIG.load();

    match config.verbosity {
        Verbosity::None => {}
        verbosity => {
            env_logger::Builder::new()
                .filter_module("mio", LevelFilter::Warn)
                .parse_env(
                    env_logger::Env::default().default_filter_or(match verbosity {
                        Verbosity::None => unreachable!(),
                        Verbosity::Error => "error",
                        Verbosity::Warn => "warn",
                        Verbosity::Info => "info",
                        Verbosity::Debug => "debug",
                        Verbosity::Trace => "trace",
                    }),
                )
                .init();
        }
    }

    let database = match config.database.as_ref() {
        Some(path) => match Database::open_file(path).await {
            Ok(x) => x,
            Err(e) => {
                error!("failed to load database file @ {path}: {e:?}");
                std::process::exit(1);
            }
        },
        None => {
            warn!("A database is not configured, using in-memory database (ephemeral). All data will be lost on process termination.");
            match Database::open_in_memory().await {
                Ok(x) => x,
                Err(e) => {
                    error!("failed to load database file in memory: {e:?}");
                    std::process::exit(1);
                }
            }
        }
    };

    //TODO: load sync

    //TODO: spawn networking

    //TODO: spawn RPC

    //TODO: start miner

    let conf: PeerConfig = serde_yaml::from_str("{}").unwrap();
    conf.print();

    test_http_client(
        &conf,
        "http://tracker.opentrackr.org:1337/announce".to_string(),
    )
    .await
    .unwrap();

    std::future::pending::<()>().await;
}
