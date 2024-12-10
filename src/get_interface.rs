use crate::client::{deserialize_response, get_random_u32};
use crate::get_torrents::{FilterOptions, Torrent};
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};

impl DelugeClient {
    /// Gather the information required for updating the web interface.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_interface(
        &mut self,
        filters: FilterOptions,
    ) -> Result<Response<Interface>, Error> {
        let method = "web.update_ui";
        let data = json!({
            "method": method,
            "params": [ [], filters ],
            "id": get_random_u32()
        });
        println!("{data}");
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interface {
    pub connected: bool,
    pub filters: Counts,
    pub stats: Stats,
    pub torrents: Option<HashMap<String, Torrent>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Counts {
    label: HashSet<(String, u64)>,
    owner: HashSet<(String, u64)>,
    state: HashSet<(String, u64)>,
    tracker_host: HashSet<(String, u64)>,
}

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

#[cfg(test)]
mod tests {

    use crate::get_torrents::FilterOptions;
    use crate::options::get_test_options;
    use crate::DelugeClient;
    use log::trace;
    use rogue_logging::{Error, Logger};

    #[tokio::test]
    async fn get_interface() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options = get_test_options()?;
        let mut client = DelugeClient::from_options(options.clone());
        let filters = FilterOptions {
            label: Some(vec!["linux".to_owned()]),
            ..FilterOptions::default()
        };

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let response = client.get_interface(filters).await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let result = response.get_result("get_interface")?;
        assert!(result.torrents.is_some());
        Ok(())
    }
}
