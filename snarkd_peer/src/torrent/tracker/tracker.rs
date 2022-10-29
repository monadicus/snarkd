use crate::torrent::{AnnounceRequest, AnnounceResponse, ScrapeResponse};
use async_trait::async_trait;
use std::error;

#[async_trait]
pub trait Tracker {
    async fn scrape(
        &self,
        info_hashes: Vec<String>,
    ) -> Result<ScrapeResponse, Box<dyn error::Error>>;

    async fn announce(
        &self,
        req: AnnounceRequest,
    ) -> Result<AnnounceResponse, Box<dyn error::Error>>;
}
