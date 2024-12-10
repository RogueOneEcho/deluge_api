use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    dht_nodes: u64,
    download_protocol_rate: f64,
    download_rate: f64,
    external_ip: String,
    free_space: u64,
    has_incoming_connections: u64,
    max_download: f64,
    max_num_connections: i64,
    max_upload: f64,
    num_connections: u64,
    upload_protocol_rate: f64,
    upload_rate: f64,
}
