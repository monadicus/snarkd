use snarkd_common::Digest;

use crate::proto::{packet::PacketBody, *};

impl PacketBody {
    pub fn into_blocks(self) -> Option<Blocks> {
        match self {
            PacketBody::Blocks(x) => Some(x),
            _ => None,
        }
    }

    pub fn into_transactions(self) -> Option<Transactions> {
        match self {
            PacketBody::Transactions(x) => Some(x),
            _ => None,
        }
    }

    pub fn into_digests(self) -> Option<Vec<Digest>> {
        match self {
            PacketBody::Digests(x) => Some(x.hashes),
            _ => None,
        }
    }

    pub fn into_ping_pong(self) -> Option<u64> {
        match self {
            PacketBody::PingPong(x) => Some(x),
            _ => None,
        }
    }

    pub fn into_peer_list(self) -> Option<PeerList> {
        match self {
            PacketBody::Peers(x) => Some(x),
            _ => None,
        }
    }

    pub fn into_introduction(self) -> Option<Introduction> {
        match self {
            PacketBody::Introduction(x) => Some(x),
            _ => None,
        }
    }

    pub fn into_error_message(self) -> Option<String> {
        match self {
            PacketBody::ErrorMessage(x) => Some(x),
            _ => None,
        }
    }
}
