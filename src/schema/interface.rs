use crate::schema::Torrent;
use crate::schema::{Counts, Stats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Interface {
    pub connected: bool,
    pub filters: Counts,
    pub stats: Stats,
    pub torrents: Option<HashMap<String, Torrent>>,
}
