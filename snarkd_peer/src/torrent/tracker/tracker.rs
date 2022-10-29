use crate::torrent::{AnnounceRequest, AnnounceResponse, ScrapeResponse};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Tracker {
    async fn scrape(&self, info_hashes: Vec<String>) -> Result<ScrapeResponse>;

    async fn announce(&self, req: AnnounceRequest) -> Result<AnnounceResponse>;
}
