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
    pub label: HashSet<(String, u64)>,
    pub owner: HashSet<(String, u64)>,
    pub state: HashSet<(String, u64)>,
    pub tracker_host: HashSet<(String, u64)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub dht_nodes: u64,
    pub download_protocol_rate: f64,
    pub download_rate: f64,
    pub external_ip: String,
    pub free_space: u64,
    pub has_incoming_connections: u64,
    pub max_download: f64,
    pub max_num_connections: i64,
    pub max_upload: f64,
    pub num_connections: u64,
    pub upload_protocol_rate: f64,
    pub upload_rate: f64,
}

#[cfg(test)]
mod tests {

    use crate::get_torrents::FilterOptions;
    use crate::DelugeClient;
    use crate::DelugeClientOptions;
    use log::trace;
    use rogue_config::{OptionsProvider, YamlOptionsProvider};
    use rogue_logging::{Error, LoggerBuilder};

    #[tokio::test]
    async fn get_interface() -> Result<(), Error> {
        // Arrange
        let _ = LoggerBuilder::new().create();
        let options: DelugeClientOptions = YamlOptionsProvider::get()?;
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
