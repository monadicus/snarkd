use super::Tracker;
use crate::torrent::{
    bencode_bytes_to_json, uri_encode_hash, AnnounceEvent, AnnounceRequest, AnnounceResponse,
    ScrapeResponse, TrackerResult,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::debug;
use url::Url;

pub struct TrackerHTTP {
    url: Url,
}

impl TrackerHTTP {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

fn kv_to_query(kvs: Vec<(&str, Option<String>)>) -> String {
    kvs.iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| format!("{k}={v}")))
        .collect::<Vec<String>>()
        .join("&")
}

#[async_trait]
impl Tracker for TrackerHTTP {
    async fn scrape(&self, info_hashes: Vec<String>) -> Result<ScrapeResponse> {
        let mut u = self.url.clone();

        // replace "announce" in path with "scrape" according to https://www.bittorrent.org/beps/bep_0048.html
        u.set_path(&u.path().replace("announce", "scrape"));

        u.set_query(Some(&kv_to_query(
            info_hashes
                .iter()
                .map(|hash| ("info_hash", uri_encode_hash(hash.as_bytes())))
                .collect(),
        )));

        let bytes = reqwest::get(u.to_string()).await?.bytes().await?;
        let blob = bencode_bytes_to_json(&bytes)?;
        debug!("received data from tracker scrape {blob}");
        match serde_json::from_str(&blob)? {
            TrackerResult::Ok(res) => Ok(res),
            TrackerResult::Err(err) => Err(anyhow!(String::from_utf8(err.failure_reason)?)),
        }
    }

    async fn announce(&self, req: AnnounceRequest) -> Result<AnnounceResponse> {
        let mut u = self.url.clone();

        u.set_query(Some(&kv_to_query(vec![
            ("info_hash", uri_encode_hash(req.info_hash.as_bytes())),
            ("peer_id", Some(req.peer_id)),
            ("port", Some(req.port.to_string())),
            ("ip", req.ip.map(|i| i.to_string())),
            ("downloaded", req.downloaded.map(|i| i.to_string())),
            ("uploaded", req.uploaded.map(|i| i.to_string())),
            ("left", req.left.map(|i| i.to_string())),
            (
                "event",
                if req.event != AnnounceEvent::None {
                    Some(req.event.to_string())
                } else {
                    None
                },
            ),
        ])));

        let bytes = reqwest::get(u.to_string()).await?.bytes().await?;
        let blob = bencode_bytes_to_json(&bytes)?;
        debug!("received data from tracker announce {blob}");
        match serde_json::from_str(&blob)? {
            TrackerResult::Ok(res) => Ok(res),
            TrackerResult::Err(err) => Err(anyhow!(String::from_utf8(err.failure_reason)?)),
        }
    }
}
