/* use bip_dht::PeerId;
use bip_handshake::{transports::TcpTransport, HandshakerBuilder, HandshakerConfig};
use bip_peer::PeerManagerBuilder;
use bip_util::sha::ShaHash;
use bip_utracker::{
    announce::{AnnounceEvent, ClientState},
    ClientRequest, TrackerClient,
};
use tokio_core::reactor::Core; */

use serde_json::json;

use crate::{
    config::PeerConfig,
    torrent::{AnnounceRequest, Tracker, TrackerHTTP},
};

/* pub fn wip_bip_client(peer_id: String, info_hash: String, tracker: std::net::SocketAddr) {
    let mut core = Core::new().unwrap();

    // Create a handshaker that can initiate connections with peers
    let handshaker = HandshakerBuilder::new()
        .with_peer_id(PeerId::from_hash(peer_id.as_bytes()).unwrap())
        .with_config(
            HandshakerConfig::default()
                .with_wait_buffer_size(0)
                .with_done_buffer_size(0),
        )
        .build(TcpTransport {}, core.handle())
        .unwrap();

    let (handshaker_send, handshaker_recv) = handshaker.into_parts();

    let (peer_manager_send, peer_manager_recv) = PeerManagerBuilder::new()
        .with_sink_buffer_capacity(0)
        .with_stream_buffer_capacity(0)
        .build(core.handle())
        .into_parts();

    let (send, recv) = std::sync::mpsc::channel();

    let mut client = TrackerClient::new("127.0.0.0:0".parse().unwrap(), handshaker).unwrap();

    let send_token = client
        .request(
            tracker,
            ClientRequest::Announce(
                ShaHash::from_bytes(info_hash.as_bytes()),
                ClientState::new(0, 0, 0, AnnounceEvent::Started),
            ),
        )
        .unwrap();
}
 */
pub async fn test_http_client(
    conf: &PeerConfig,
    tracker: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let u: url::Url = tracker.parse().unwrap();

    let tracker = TrackerHTTP::new(u);
    let scraped = tracker.scrape(vec![conf.info_hash.clone()]).await?;
    println!("scraped: {}", json!(scraped));

    let announce = tracker
        .announce(AnnounceRequest {
            peer_id: conf.peer_id.clone(),
            info_hash: conf.info_hash.clone(),
            port: 4333,
            ..Default::default()
        })
        .await?;

    println!("announce: interval: {}, min_interval: {}, complete: {}, incomplete: {}, downloaded: {}\npeers: {}",
        announce.interval, announce.min_interval, announce.complete, announce.incomplete, announce.downloaded,
        announce.peer_addrs().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ")
);

    Ok(())
}
