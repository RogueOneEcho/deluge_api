use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum State {
    Downloading,
    Seeding,
    Paused,
    Error,
    Queued,
    Checking,
}
