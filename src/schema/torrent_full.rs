use crate::schema::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentFull {
    completed_time: u64,
    distributed_copies: f64,
    download_location: String,
    download_payload_rate: f64,
    eta: f64,
    is_auto_managed: bool,
    label: String,
    last_seen_complete: u64,
    max_download_speed: i64,
    max_upload_speed: i64,
    name: String,
    num_peers: u32,
    num_seeds: u32,
    progress: f64,
    queue: i32,
    ratio: f64,
    seeds_peers_ratio: f64,
    state: State,
    time_added: u64,
    time_since_transfer: u64,
    total_done: u64,
    total_peers: u32,
    total_remaining: u64,
    total_seeds: u32,
    total_uploaded: u64,
    total_wanted: u64,
    tracker_host: String,
    upload_payload_rate: f64,
}
