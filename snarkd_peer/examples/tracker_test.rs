use snarkd_peer::{
    config::PeerConfig,
    torrent::{AnnounceRequest, TrackerHTTP},
};

pub async fn announce_scrape_tracker(
    conf: &PeerConfig,
    tracker: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let u: url::Url = tracker.parse().unwrap();

    let tracker = TrackerHTTP::new(u);

    // scrape example
    // you can use scrape to check how many peers are on the listed infohashes
    let res = tracker.scrape(vec![conf.info_hash.clone()]).await?;
    println!("scraped: {}", json!(res));

    // announce example
    // responds with a list of peers, and timing for the next announce
    let res = tracker
        .announce(AnnounceRequest {
            peer_id: conf.peer_id.clone(),
            info_hash: conf.info_hash.clone(),
            port: 4333,
            ..Default::default()
        })
        .await?;

    println!("announce: interval: {}, min_interval: {}, complete: {}, incomplete: {}, downloaded: {}\npeers: {}",
        res.interval, res.min_interval, res.complete, res.incomplete, res.downloaded,
        res.peer_addrs().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ")
);

    Ok(())
}

#[tokio::main]
fn main() {
    // parse config from yaml
    let conf: PeerConfig = serde_yaml::from_str("{}").unwrap();
    conf.print();

    announce_scrape_tracker(
        &conf,
        "http://tracker.opentrackr.org:1337/announce".to_string(),
    )
    .await
    .unwrap();
}
