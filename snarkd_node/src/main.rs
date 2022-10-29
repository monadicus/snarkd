use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use clap::Parser;
use config::{Verbosity, CONFIG};
use log::{error, info, warn, LevelFilter};
use peer_book::PeerBook;
use snarkd_network::Connection;
use snarkd_peer::announcer::AnnouncerConsumer;
use snarkd_storage::{Database, PeerDirection};

use crate::inbound_handler::InboundHandler;

mod config;
mod inbound_handler;
mod peer;
mod peer_book;

/// Snarkd Blockchain Node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

const ANNOUNCE: &str =
r#",-▄▄██▄▄,,
,▄██▀▀╙`      "╙▀█▓▄▄,
▐█╙▀▀█▄▄,     ,▄██▀▀▀▌
▐▌      "▀▓█▀▀`     ▐▌
▐▌        ║█        ▐▌
,╓▄▓█▀▀▌        ║█        ▐▌
å███▌     ▐▌        ║█        ▐▌
║▌  "▀▀▀███▌        ║█        ▐▌
║▌        ╟▌        ║█        ▐▌
,,▄▄█▌        ▐▌        ║█        ▐▌
»▄██▀▀"   ║▌        ╟▌        ║█        ▐▌
╟▌╙▀▀█▄▄, ║▌        ╟▌        ║█    ,-▄▓███▄µ,
╟▌      `▀█▌        ╟▌        ║███▀▀`        "▀▀██▄▄
╟▌        ║▌        ╟▌        ║█╙▀▀▀█▄▄,  ,▄▄█▓▀▀╙║▌
║▌        ║▌        ╟▌        ║█       "█▌`       ║▌
║▌        ╟▌        ╟▌        ║█        ╟▌        ║▌
║▌        ║▌        ╟▌        ║█        ╟▌        ║▌
║▌        ╟▌        ╟▌╙▀▀▓▄u, ║█        ╟▌        ║▌
║▌        ╟▌        ╟▌     "╙▀██        ╟▌        ║▌
║▌        ╟▌        ╟▌        ║█        ╟▌        ║▌
║▌        ╟▌        ╟▌        ║█        ╟▌        ║▌
╟▌        ║▌        ╟▌        ║█        ╟▌        ║▌
║▌        ╟▌        ╟▌        ║█        ╟▌        ║▌
║▌        ╟▌        ╟▌        ║█        ╟▌        ║▌
╟▌        ╟██▓▄,    ╟▌     ,«▄██        ╟▌        ║▌
║▌        ║▌   "▀▀█▄██▄▓█▀▀╙` ▐▌        ╟▌        ║▌
╟▌        ╟▌        ╟▌        ║▌        ╟▌        ║▌
╟▌        ╟▌        ╟▌        ║▌        ╟▌        ║▌
╟▌        ╟▌        ╟▌        ║▌        ╟▌        ║▌
╙▀▀▓▄▄,   ╟▌   ,µ▄▓▀▀`        ║▌        ╟▌        ║▌
"▀▀▀██▀▀▀^              ║▌        ╟▌        ║▌
         ║▌        ╟▌        ║▌
         ▐█        ╟▌        ║▌
          "▀▀▀█▓▄-,╟▌,-▄▄█▀▀▀
                 `╙▀▀╙`"#;

#[tokio::main]
async fn main() {
    println!("{ANNOUNCE}");
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

    let database = match config.database_file.as_ref() {
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
    let database = Arc::new(database);

    let peer_book = PeerBook::new();

    // spawn network listener
    {
        let listen_port = config.listen_port;
        let peer_book = peer_book.clone();
        let database = database.clone();
        tokio::spawn(async move {
            if let Err(e) = Connection::listen(
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, listen_port)),
                |address| InboundHandler { address },
                move |connection| {
                    info!("received connection from {}", connection.remote_addr());
                    let peer_book = peer_book.clone();
                    let database = database.clone();
                    tokio::spawn(async move {
                        if let Err(e) = peer_book
                            .discovered_peers(&*database, [connection.remote_addr()])
                            .await
                        {
                            error!(
                                "failed to discover received peer {}: {e:?}",
                                connection.remote_addr()
                            );
                            return;
                        }
                        if let Some(mut peer) = peer_book.peer_mut(&connection.remote_addr()) {
                            peer.data.last_peer_direction = PeerDirection::Inbound;
                            peer.register_connection(connection);
                            if let Err(e) = peer.data.save(&*database).await {
                                error!("failed to save received peer: {e:?}");
                            }
                        }
                    });
                },
            )
            .await
            {
                error!("failed to listen on port {listen_port}: {e:?}");
            }
        });
    }

    // load initial peers from database, if any
    info!("loading peers from database...");
    if let Err(e) = peer_book.load_saved_peers(&database).await {
        error!("failed to load peers from database: {e:?}");
    }

    // spawn peer connect/disconnect task
    {
        let peer_book = peer_book.clone();
        let database = database.clone();
        let mut timer =
            tokio::time::interval(Duration::from_secs(config.peer_sync_interval as u64));
        tokio::spawn(async move {
            loop {
                peer_book.update_peer_connections(&database).await;
                timer.tick().await;
            }
        });
    }

    // spawn announcer
    if config.enable_tracker_announce {
        info!("preparing tracker announce...");
        let peer_config = config.tracker.clone();
        let inbound_port = config.inbound_port.unwrap_or(config.listen_port);
        let maximum_peers = config.maximum_connection_count;
        let peer_book = peer_book.clone();
        let database = database.clone();
        tokio::spawn(async move {
            #[derive(Clone)]
            struct PeerReceiver {
                peer_book: PeerBook,
                database: Arc<Database>,
                maximum_peers: usize,
            }

            impl AnnouncerConsumer for PeerReceiver {
                fn peers_needed(&self) -> usize {
                    (self.maximum_peers * 2).saturating_sub(self.peer_book.connected_peer_count())
                }

                fn receive_peers(&self, peers: Vec<SocketAddr>) {
                    if peers.is_empty() {
                        return;
                    }

                    let database = self.database.clone();
                    let peer_book = self.peer_book.clone();

                    tokio::spawn(async move {
                        if let Err(e) = peer_book.discovered_peers(&*database, peers).await {
                            error!("failed storing discovered peers: {e:?}");
                        }
                    });
                }
            }

            snarkd_peer::announcer::run(
                peer_config,
                inbound_port,
                PeerReceiver {
                    peer_book,
                    database,
                    maximum_peers,
                },
            )
            .await;
        });
    }

    //TODO: peer introduction send/recv

    //TODO: peer blacklisting (including blacklisting ourselves)

    //TODO: spawn peer pinger

    //TODO: spawn peer syncer

    //TODO: spawn RPC

    //TODO: start miner

    std::future::pending::<()>().await;
}
