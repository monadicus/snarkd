use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScrapeMetadata {
    /// The number of active peers that have completed downloading.
    pub complete: i32,
    /// The number of active peers that have not completed downloading.
    pub incomplete: i32,
    /// The number of peers that have ever completed downloading.
    pub downloaded: i32,
}

/// As defined in https://www.bittorrent.org/beps/bep_0048.html
/// The successful response to a scrape request is a dictionary where
/// keys are 20-byte InfoHashes, values are `ScrapeMetadata`
#[derive(Serialize, Deserialize, Debug)]
pub struct ScrapeResponse {
    pub files: HashMap<String, ScrapeMetadata>,
}
