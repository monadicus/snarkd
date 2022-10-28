/* use bip_dht::PeerId;
use bip_handshake::{transports::TcpTransport, HandshakerBuilder, HandshakerConfig};
use bip_peer::PeerManagerBuilder;
use bip_util::sha::ShaHash;
use bip_utracker::{
    announce::{AnnounceEvent, ClientState},
    ClientRequest, TrackerClient,
};
use tokio_core::reactor::Core; */

use crate::config::PeerConfig;

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
pub async fn http_client(
    conf: &PeerConfig,
    tracker: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut u: url::Url = tracker.parse().unwrap();
    u.set_query(Some(&format!(
        "info_hash={}&peer_id={}&port={}",
        conf.info_hash
            .as_bytes()
            .chunks(2)
            .map(|chunk| format!("%{}", std::str::from_utf8(chunk).unwrap()))
            .collect::<Vec<String>>()
            .join(""),
        conf.peer_id,
        4333,
    )));
    println!("Announce Query {}", u.to_string());
    let resp = reqwest::get(u.to_string()).await?.text().await?;
    println!("{:#?}", resp);

    u.set_path("/scrape");
    u.set_query(Some(&format!(
        "info_hash={}",
        conf.info_hash
            .as_bytes()
            .chunks(2)
            .map(|chunk| format!("%{}", std::str::from_utf8(chunk).unwrap()))
            .collect::<Vec<String>>()
            .join(""),
    )));
    println!("Scrape Query {}", u.to_string());
    let resp = reqwest::get(u.to_string()).await?.text().await?;
    println!("{:#?}", resp);
    Ok(())
}
