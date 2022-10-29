use std::{net::SocketAddr, time::Duration};

use chrono::{DateTime, Utc};
use log::{info, warn};
use snarkd_network::Connection;
use snarkd_storage::PeerData;
use tokio::task::JoinHandle;

use crate::inbound_handler::InboundHandler;

enum ConnectionState {
    Connected(Connection),
    Connecting(JoinHandle<()>),
    Disconnected,
}

pub struct Peer {
    pub address: SocketAddr,
    connection: ConnectionState,
    pub data: PeerData,
    // if true, we have not saved state to disk yet
    pub dirty: bool,
    recent_failures: Vec<DateTime<Utc>>,
}

const FAILURE_EXPIRY_TIME: Duration = Duration::from_secs(15 * 60);
const FAILURE_THRESHOLD: usize = 5;
pub const MAX_PEER_INACTIVITY: Duration = Duration::from_secs(30);

impl Peer {
    pub fn new(address: SocketAddr, data: PeerData) -> Self {
        Self {
            address,
            connection: ConnectionState::Disconnected,
            data,
            recent_failures: vec![],
            dirty: false,
        }
    }

    pub fn recent_failures(&mut self) -> usize {
        let now = Utc::now();
        if self.recent_failures.len() >= FAILURE_THRESHOLD {
            self.recent_failures = self
                .recent_failures
                .iter()
                .filter(|x| {
                    now.signed_duration_since(**x)
                        < chrono::Duration::from_std(FAILURE_EXPIRY_TIME).unwrap()
                })
                .copied()
                .collect();
        }
        self.recent_failures.len()
    }

    pub fn is_inactive(&self) -> bool {
        if !self.is_connected() {
            return false;
        }
        let last_seen = self.data.last_seen;
        if let Some(last_seen) = last_seen {
            Utc::now() - last_seen > chrono::Duration::from_std(MAX_PEER_INACTIVITY).unwrap()
        } else {
            false
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.connection, ConnectionState::Connected(_))
    }

    pub fn disconnect(&mut self) {
        if matches!(self.connection, ConnectionState::Disconnected) {
            return;
        }
        info!("Disconnecting from {}", self.address);
        self.connection = ConnectionState::Disconnected;
    }

    pub fn connect(&mut self, output: impl FnOnce(Option<Connection>) + Send + Sync + 'static) {
        let address = self.address;
        let handle = tokio::spawn(async move {
            match Connection::connect(address, InboundHandler { address }).await {
                Ok(connection) => output(Some(connection)),
                Err(e) => {
                    warn!("failed to connect to peer {address}: {e:?}");
                    output(None)
                }
            }
        });
        self.connection = ConnectionState::Connecting(handle);
    }

    pub fn register_failed_connection(&mut self) {
        self.data.connection_fail_count += 1;
        self.dirty = true;
    }

    pub fn register_connection(&mut self, connection: Connection) {
        assert_eq!(connection.remote_addr(), self.address);
        self.connection = ConnectionState::Connected(connection);
        self.data.connection_success_count += 1;
        self.data.last_connected = Some(Utc::now());
        self.dirty = true;
    }

    pub fn judge_bad(&mut self) -> bool {
        // warn!("Peer {} has a low quality score; disconnecting.", self.address);
        self.recent_failures() >= FAILURE_THRESHOLD || self.is_inactive()
    }
}
