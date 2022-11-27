use std::{net::SocketAddr, sync::Arc};

use crate::{config::CONFIG, peer::Peer, rpc::RpcChannels};
use anyhow::Result;
use dashmap::{
    mapref::{
        entry::Entry,
        multiple::{RefMulti, RefMutMulti},
        one::{Ref, RefMut},
    },
    DashMap,
};
use itertools::Itertools;
use log::{debug, error, info, trace, warn};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use snarkd_storage::{Database, PeerData, PeerDirection};

#[derive(Clone)]
pub struct PeerBook {
    rpc_channels: Arc<RpcChannels>,
    peers: Arc<DashMap<SocketAddr, Peer>>,
}

impl PeerBook {
    pub fn new(rpc_channels: Arc<RpcChannels>) -> Self {
        Self {
            rpc_channels,
            peers: Default::default(),
        }
    }

    pub async fn load_saved_peers(&self, db: &Database) -> Result<()> {
        for peer_data in PeerData::load_all(db).await? {
            let mut peer = self.peers.entry(peer_data.address).or_insert_with(|| {
                Peer::new(peer_data.address, peer_data, self.rpc_channels.clone())
            });
            peer.data.merge_from(&peer_data);
        }
        Ok(())
    }

    pub async fn discovered_peers(
        &self,
        db: &Database,
        peers: impl IntoIterator<Item = SocketAddr>,
    ) -> Result<()> {
        for address in peers {
            match self.peers.entry(address) {
                Entry::Occupied(_) => {
                    trace!("peer {address} rediscovered");
                }
                Entry::Vacant(slot) => {
                    debug!("peer {address} discovered");
                    let peer_data = PeerData::new(address);
                    slot.insert(Peer::new(address, peer_data, self.rpc_channels.clone()));
                    peer_data.save(db).await?;
                }
            }
        }
        Ok(())
    }

    pub fn peer(&self, address: &SocketAddr) -> Option<Ref<'_, SocketAddr, Peer>> {
        self.peers.get(address)
    }

    pub fn peer_mut(&self, address: &SocketAddr) -> Option<RefMut<'_, SocketAddr, Peer>> {
        self.peers.get_mut(address)
    }

    pub fn connected_peers(&self) -> impl Iterator<Item = RefMulti<'_, SocketAddr, Peer>> {
        self.peers.iter().filter(|x| x.is_connected())
    }

    fn disconnected_peers(&self) -> impl Iterator<Item = RefMulti<'_, SocketAddr, Peer>> {
        self.peers.iter().filter(|x| !x.is_connected())
    }

    pub fn connected_peers_mut(&self) -> impl Iterator<Item = RefMutMulti<'_, SocketAddr, Peer>> {
        self.peers.iter_mut().filter(|x| x.is_connected())
    }

    pub fn connected_peer_count(&self) -> usize {
        self.connected_peers().count()
    }

    /// disconnect from `count` peers at random
    pub fn disconnect_from_peers(&self, count: usize) {
        if count == 0 {
            return;
        }
        info!("Disconnecting {count} peers");
        let connected_peers = self
            .connected_peers()
            .map(|x| *x.key())
            .choose_multiple(&mut thread_rng(), count);
        for peer in connected_peers {
            if let Some(mut peer) = self.peers.get_mut(&peer) {
                peer.disconnect();
            }
        }
    }

    fn connect_to_known_peer(&self, address: SocketAddr) {
        let peers = self.peers.clone();
        // this doesnt deadlock in DashMap because there is a tokio::spawn deferring the actual connection
        let mut peer = match self.peers.get_mut(&address) {
            Some(peer) => peer,
            None => return,
        };

        peer.connect(self.clone(), move |connection| {
            if let Some(mut peer) = peers.get_mut(&address) {
                match connection {
                    None => peer.register_failed_connection(),
                    Some(connection) => {
                        if peer.is_connected() {
                            warn!("peer {address} was already connected during peer connection, they must have connected to us first");
                            return;
                        }
                        peer.register_connection(PeerDirection::Outbound, connection);
                        info!("connected to peer {}", peer.address);
                    }
                }
            }
        });
    }

    pub fn connect_to_peers(&self, count: usize) {
        if count == 0 {
            return;
        }
        debug!("Looking for {count} new peer connections");

        let target_peers = {
            // Floored if count is odd.
            let random_count = count / 2;
            let random_picks = self
                .disconnected_peers()
                .map(|x| x.address)
                .choose_multiple(&mut thread_rng(), random_count);

            let mut candidates = self.disconnected_peers().collect::<Vec<_>>();
            candidates.sort_unstable_by(|x, y| y.data.block_height.cmp(&x.data.block_height));

            candidates.truncate(count - random_count);
            candidates
                .into_iter()
                .map(|x| x.address)
                .chain(random_picks)
                .unique()
                .collect::<Vec<_>>()
        };

        if target_peers.is_empty() {
            return;
        }

        for target_peer in target_peers {
            self.connect_to_known_peer(target_peer);
        }
    }

    /// Connects and disconnects peers to maintain the appropriate peer counts
    /// Does not search for new peers.
    pub async fn update_peer_connections(&self, database: &Database) {
        //todo: do we need connecting_peers
        let active_peer_count = self.connected_peer_count();
        debug!(
            "Connected to {} peer{}",
            active_peer_count,
            if active_peer_count == 1 { "" } else { "s" }
        );

        // disconnect bad peers
        for mut peer in self.connected_peers_mut() {
            if peer.judge_bad() {
                peer.disconnect();
            }
        }

        let active_peer_count = self.connected_peer_count();
        let config = CONFIG.load();

        let to_disconnect = active_peer_count.saturating_sub(config.maximum_connection_count);
        let to_connect = config
            .minimum_connection_count
            .saturating_sub(active_peer_count);

        self.disconnect_from_peers(to_disconnect);
        self.connect_to_peers(to_connect);

        self.save_peers(database).await;
    }

    async fn save_peers(&self, database: &Database) {
        for mut peer in self.peers.iter_mut() {
            if peer.dirty {
                if let Err(e) = peer.save(database).await {
                    error!("failed to save peer data to database: {e:?}");
                }
            }
        }
    }
}
