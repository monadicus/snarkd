use std::net::SocketAddr;
use std::time::Duration;

use log::error;

use crate::config::PeerConfig;

use crate::torrent::{AnnounceRequest, Tracker, TrackerHTTP};

pub trait AnnouncerConsumer: Clone + Send + Sync + 'static {
    fn peers_needed(&self) -> usize;

    fn receive_peers(&self, peers: Vec<SocketAddr>);
}

//todo(max): graceful termination
pub async fn run(config: PeerConfig, inbound_port: u16, consumer: impl AnnouncerConsumer) {
    let request = AnnounceRequest {
        info_hash: config.info_hash.clone(),
        peer_id: config.peer_id.clone(),
        port: inbound_port,
        // num_want: Some(max_peers),
        ..Default::default()
    };
    for tracker in config.trackers.into_iter().map(|url| TrackerHTTP::new(url)) {
        let mut request = request.clone();
        let consumer = consumer.clone();
        tokio::spawn(async move {
            request.num_want = Some(consumer.peers_needed() as i64);
            let response = match tracker.announce(request.clone()).await {
                Ok(x) => x,
                Err(e) => {
                    error!(
                        "failed initial announce to tracker {}: {e:?}",
                        tracker.url()
                    );
                    return;
                }
            };
            let peer_addrs = response.peer_addrs();
            consumer.receive_peers(peer_addrs.into_iter().map(SocketAddr::V4).collect());

            let mut timer = tokio::time::interval(Duration::from_secs(response.interval as u64));
            loop {
                timer.tick().await;
                request.num_want = Some(consumer.peers_needed() as i64);
                let response = match tracker.announce(request.clone()).await {
                    Ok(x) => x,
                    Err(e) => {
                        error!(
                            "failed initial announce to tracker {}: {e:?}",
                            tracker.url()
                        );
                        return;
                    }
                };
                let peer_addrs = response.peer_addrs();
                consumer.receive_peers(peer_addrs.into_iter().map(SocketAddr::V4).collect());
                // it's expected that trackers don't change their intervals, but we might want to handle that anyways
            }
        });
    }
}
