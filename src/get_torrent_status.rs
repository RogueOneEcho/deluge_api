use crate::client::{deserialize_response, get_random_u32};
use crate::State;
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
    pub completed_time: u64,
    pub distributed_copies: f64,
    pub download_location: String,
    pub download_payload_rate: f64,
    pub eta: f64,
    pub is_auto_managed: bool,
    pub label: String,
    pub last_seen_complete: u64,
    pub max_download_speed: i64,
    pub max_upload_speed: i64,
    pub name: String,
    pub num_peers: u32,
    pub num_seeds: u32,
    pub progress: f64,
    pub queue: i32,
    pub ratio: f64,
    pub seeds_peers_ratio: f64,
    pub state: State,
    pub time_added: u64,
    pub time_since_transfer: u64,
    pub total_done: u64,
    pub total_peers: u32,
    pub total_remaining: u64,
    pub total_seeds: u32,
    pub total_uploaded: u64,
    pub total_wanted: u64,
    pub tracker_host: String,
    pub upload_payload_rate: f64,
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
