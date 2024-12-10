use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Counts {
    label: HashSet<(String, u64)>,
    owner: HashSet<(String, u64)>,
    state: HashSet<(String, u64)>,
    tracker_host: HashSet<(String, u64)>,
}
