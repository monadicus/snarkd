use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Serialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnnounceEvent {
    #[default]
    None,
    Completed,
    Started,
    Stopped,
}

impl std::fmt::Display for AnnounceEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Clone, Debug, Default, Serialize)]
/// Announce request outlined in https://www.bittorrent.org/beps/bep_0015.html
pub struct AnnounceRequest {
    pub info_hash: String,
    pub peer_id: String,
    pub port: u16,
    pub event: AnnounceEvent,
    pub downloaded: Option<i64>,
    pub left: Option<i64>,
    pub uploaded: Option<i64>,
    #[serde(rename = "numwant")]
    pub num_want: Option<i64>,
    pub ip: Option<Ipv4Addr>,
}

#[derive(Deserialize, Debug)]
/// Announce response outlined in https://www.bittorrent.org/beps/bep_0015.html
pub struct AnnounceResponse {
    /// Do not announce again until `interval` seconds have passed or an event has occurred.
    pub interval: i32,
    pub complete: i32,
    pub downloaded: i32,
    pub incomplete: i32,
    #[serde(rename = "min interval")]
    pub min_interval: i32,
    pub peers: Vec<u8>,
}

impl AnnounceResponse {
    /// Parse peer addrs from raw bytes
    pub fn peer_addrs(&self) -> Vec<SocketAddrV4> {
        if self.peers.len() % 6 != 0 {
            return vec![];
        }

        self.peers
            .chunks(6)
            .map(|bytes| {
                SocketAddrV4::new(
                    Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]),
                    ((bytes[4] as u16) << 8) | bytes[5] as u16,
                )
            })
            .collect()
    }
}
