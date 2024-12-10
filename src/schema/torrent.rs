use crate::schema::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    label: String,
    name: String,
    progress: f64,
    save_path: String,
    state: State,
    total_remaining: u64,
}
