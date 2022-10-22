use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Peer {
    pub address: SocketAddr,
    pub block_height: u32,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_connected: Option<DateTime<Utc>>,
    pub blocks_synced_to: u32,
    pub blocks_synced_from: u32,
    pub blocks_received_from: u32,
    pub blocks_sent_to: u32,
    pub connection_attempt_count: u64,
    pub connection_success_count: u64,
    pub connection_transient_fail_count: u64,
}
