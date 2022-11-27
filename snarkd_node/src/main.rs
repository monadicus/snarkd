use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use clap::Parser;
use config::{CONFIG, NODE_ID};
use log::{debug, error, info, warn, LevelFilter};
use peer_book::PeerBook;
use snarkd_common::config::Verbosity;
use snarkd_network::Connection;
use snarkd_peer::announcer::AnnouncerConsumer;
use snarkd_rpc::server::websocket_server;
use snarkd_storage::{Database, PeerDirection};
use tokio::{net::TcpListener, sync::oneshot, time::MissedTickBehavior};

use crate::{inbound_handler::InboundHandler, peer::PEER_PING_INTERVAL};

mod config;
mod inbound_handler;
mod peer;
mod peer_book;
mod rpc;

/// Snarkd Blockchain Node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

const ANNOUNCE: &str = r#"                            ,-▄▄██▄▄,,
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
                                 "▀▀▀█▓▄-,╟▌,-▄▄█▀▀▀"
                                        `╙▀▀╙`"#;

#[tokio::main]
async fn main() {
    println!("{ANNOUNCE}");
    lazy_static::initialize(&CONFIG);

    let config = CONFIG.load();
    let has_rpc = config.rpc_port != 0;

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
    let rpc_channels = Arc::new(rpc::RpcChannels::new());

    let peer_book = PeerBook::new();

    // spawn network listener
    {
        let listen_port = config.listen_port;
        let listen_ip = config.listen_ip;
        let peer_book = peer_book.clone();
        let database = database.clone();
        let rpc_channels = rpc_channels.clone();
        tokio::spawn(async move {
            let listener =
                match TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(listen_ip, listen_port)))
                    .await
                {
                    Ok(e) => e,
                    Err(e) => {
                        error!("failed to bind for inbound connections: {e:?}");
                        return;
                    }
                };

            loop {
                let rpc_channels = rpc_channels.clone();

                let (stream, address) = match listener.accept().await {
                    Ok(x) => x,
                    Err(e) => {
                        error!("failed to accept inbound connection {e:?}");
                        continue;
                    }
                };

                if has_rpc {
                    rpc_channels.peer_message(rpc::PeerMessage::Connect(address));
                }

                let (intro_sender, intro_receiver) =
                    oneshot::channel::<snarkd_network::proto::Introduction>();
                let handler = InboundHandler::new(address, peer_book.clone(), Some(intro_sender));
                let (reader, writer) = stream.into_split();
                let connection = Connection::accept(reader, writer, address, handler);

                let peer_book = peer_book.clone();
                let database = database.clone();

                let on_disconnect = {
                    let rpc_channels = rpc_channels.clone();
                    move || {
                        if has_rpc {
                            rpc_channels.peer_message(rpc::PeerMessage::Disconnect(address));
                        }
                    }
                };

                tokio::spawn(async move {
                    let introduction = match intro_receiver.await {
                        Ok(x) => x,
                        Err(_) => {
                            debug!("failed to receive introduction from inbound peer");
                            on_disconnect();
                            return;
                        }
                    };
                    if introduction.instance_id == NODE_ID.as_bytes() {
                        debug!("self referential connection closing");
                        on_disconnect();
                        drop(connection);
                        return;
                    }
                    let mut remote_addr = connection.remote_addr();
                    remote_addr.set_port(introduction.inbound_port as u16);
                    info!("received connection from {}", remote_addr);

                    if let Err(e) = peer_book.discovered_peers(&database, [remote_addr]).await {
                        error!(
                            "failed to discover received peer {}: {e:?}",
                            connection.remote_addr()
                        );
                        on_disconnect();
                        return;
                    }
                    if let Some(mut peer) = peer_book.peer_mut(&remote_addr) {
                        peer.register_connection(PeerDirection::Inbound, connection);
                        if has_rpc {
                            rpc_channels.peer_message(rpc::PeerMessage::Handshake(peer.data));
                        }

                        if let Err(e) = peer.save(&database).await {
                            error!("failed to save received peer: {e:?}");
                        }
                    }
                });
            }
        });
    }

    // load initial peers from database, if any
    info!("loading peers from database...");
    if let Err(e) = peer_book.load_saved_peers(&database).await {
        error!("failed to load peers from database: {e:?}");
    }

    // spawn peer connect/disconnect task (also saves peers if dirty)
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
                        if let Err(e) = peer_book.discovered_peers(&database, peers).await {
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
    } else if let Err(e) = peer_book
        .discovered_peers(&database, config.tracker.peers.iter().copied())
        .await
    {
        error!("failed to add in raw tracker peers: {e:?}");
    }

    // spawn peer pinger
    {
        let peer_book = peer_book.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(PEER_PING_INTERVAL);
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                for peer in peer_book.connected_peers() {
                    peer.start_ping(peer_book.clone());
                }
                interval.tick().await;
            }
        });
    }

    //TODO: spawn peer syncer

    let rpc_handle = if has_rpc {
        let rpc_addr = SocketAddr::new(config.rpc_ip.into(), config.rpc_port);
        let rpc_module = rpc::SnarkdRpc {
            peer_book,
            channels: rpc_channels,
        }
        .module();

        match websocket_server(rpc_module, rpc_addr).await {
            Ok((addr, handle)) => {
                info!("json rpc listening on ws://{}", addr);
                Some(handle)
            }
            Err(e) => {
                error!("failed to start json rpc on {rpc_addr}: {e:?}");
                None
            }
        }
    } else {
        None
    };

    //TODO: start miner

    tokio::select! {
        _ = std::future::pending::<()>() => {
            info!("all pending tasks finished... somehow");
        },
        _ = tokio::signal::ctrl_c() => {
            info!("detected interrupt");
        },
    };

    if let Some(rpc_handle) = rpc_handle {
        info!("stopping rpc server...");
        if let Err(e) = rpc_handle.stop() {
            error!("failed stopping json rpc: {e:?}");
        }
    }
}
