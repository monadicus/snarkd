use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PeerConfig {
    /// Bittorrent Peer id, defaults to random 20 bytes
    pub peer_id: String,
    /// Not used at the moment as it's identical to the peer's port.
    pub client_port: u16,
    /// Info Hash for finding peers
    pub info_hash: String,
    /// List of trackers to find peers from. Leave empty to disable tracker based peer discovery
    pub trackers: Vec<url::Url>,
    // List of initial peers to connect to (via bittorrent)
    pub peers: Vec<SocketAddr>,
}

impl Default for PeerConfig {
    fn default() -> Self {
        Self {
            peer_id: generate_peer_id(),
            client_port: 0,
            info_hash: default_info_hash(),
            trackers: default_trackers(),
            peers: vec![],
        }
    }
}

/// Peer ids are generated at random
fn generate_peer_id() -> String {
    let bytes = hex::encode_upper((0..6).map(|_| rand::random::<u8>()).collect::<Vec<u8>>());
    format!("-MD0001-{bytes}")
}

fn default_info_hash() -> String {
    "000000F214C636F3EBD358EC783C6E8A91BF81AE".to_string()
}

fn default_trackers() -> Vec<url::Url> {
    vec![
        "http://tracker.opentrackr.org:1337/announce
",
    ]
    .iter()
    .map(|u| url::Url::parse(u).unwrap_or_else(|_| panic!("Error parsing default tracker {u}")))
    .collect()
}

#[derive(Debug)]
pub enum PeerConfigError {
    InvalidInfoHash(String),
    InvalidTracker(String),
    InvalidPeerId(String),
}

fn validate_tracker(tracker: &url::Url) -> Result<(), String> {
    match tracker.scheme() {
        "http" => (),
        proto => return Err(format!("unsupported protocol '{proto}'")),
    };

    if !tracker.has_host() {
        return Err("url is missing a host".to_string());
    }

    if tracker.port_or_known_default().is_none() {
        return Err("url is missing a port".to_string());
    }

    Ok(())
}

fn validate_hash(hash: &str) -> Result<(), String> {
    // info_hash validation
    if !hash.is_ascii() {
        return Err("hash is not ascii".to_string());
    }

    if hash.len() != 40 {
        return Err("hash is too short (Must be 40 hex characters)".to_string());
    }

    if hex::decode(hash).is_err() {
        return Err("hash is not hex digits".to_string());
    }

    Ok(())
}

impl PeerConfig {
    pub fn print(&self) {
        println!("{:#?}", self);
    }

    pub fn validate(&self) -> Result<(), PeerConfigError> {
        // info_hash validation
        if let Err(err) = validate_hash(&self.info_hash) {
            return Err(PeerConfigError::InvalidInfoHash(format!(
                "invalid peer info hash: {err}"
            )));
        }

        // peer id validation
        if self.peer_id.len() != 20 {
            return Err(PeerConfigError::InvalidPeerId(
                "invalid peer ID: length must be 20".to_string(),
            ));
        }

        // tracker validation
        if let Some((tracker, err)) = self.trackers.iter().find_map(|t| {
            if let Err(err) = validate_tracker(t).map_err(|err| (t, err)) {
                Some(err)
            } else {
                None
            }
        }) {
            return Err(PeerConfigError::InvalidInfoHash(format!(
                "invalid tracker {tracker}: {err}"
            )));
        }

        Ok(())
    }
}
