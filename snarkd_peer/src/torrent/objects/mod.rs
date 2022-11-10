use serde::Deserialize;
mod scrape;
pub use scrape::*;

mod announce;
pub use announce::*;

#[derive(Deserialize, Debug)]
pub struct TrackerFailure {
    #[serde(rename = "failure reason")]
    pub failure_reason: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TrackerResult<T> {
    Ok(T),
    Err(TrackerFailure),
}
