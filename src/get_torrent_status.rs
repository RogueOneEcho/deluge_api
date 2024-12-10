use crate::client::{deserialize_response, get_random_u32};
use crate::state::State;
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

impl DelugeClient {
    /// Get the status for a torrent, filtered by status keys.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_torrent_status(&mut self, id: &str) -> Result<Response<Torrent>, Error> {
        let method = "web.get_torrent_status";
        let data = json!({
            "method": method,
            "params": [ id, [] ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
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

#[cfg(test)]
mod tests {
    use crate::DelugeClient;
    use log::trace;
    use rogue_logging::{Error, Logger};

    use crate::options::get_test_options;

    #[tokio::test]
    async fn get_torrent_status() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options = get_test_options()?;
        let mut client = DelugeClient::from_options(options.clone());

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let id = options
            .torrent_id
            .expect("example torrent_id should be set");
        let response = client.get_torrent_status(&id).await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let _result = response.get_result("get_torrent_status")?;
        Ok(())
    }
}
