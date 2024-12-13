use crate::client::{deserialize_response, get_random_u32};
use crate::get_interface::Interface;
use crate::State;
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

impl DelugeClient {
    /// Get all torrents matching the filter
    ///
    /// This is a wrapper for `get_interface()`
    pub async fn get_torrents(
        &mut self,
        filters: FilterOptions,
    ) -> Result<Response<HashMap<String, Torrent>>, Error> {
        let method = "web.update_ui";
        let data = json!({
            "method": method,
            "params": [ [], filters ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        let response = deserialize_response::<Interface>(method, response).await?;
        Ok(Response {
            status_code: response.status_code,
            result: response.result.map(|x| x.torrents.unwrap_or_default()),
            error: response.error,
            id: response.id,
        })
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FilterOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_host: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub label: String,
    pub name: String,
    pub progress: f64,
    pub save_path: String,
    pub state: State,
    pub total_remaining: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DelugeClient;
    use crate::DelugeClientOptions;
    use log::trace;
    use rogue_config::{OptionsProvider, YamlOptionsProvider};
    use rogue_logging::{Error, Logger};

    #[tokio::test]
    async fn get_torrents() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options: DelugeClientOptions = YamlOptionsProvider::get()?;
        let mut client = DelugeClient::from_options(options.clone());
        let filters = FilterOptions {
            label: Some(vec!["linux".to_owned()]),
            ..FilterOptions::default()
        };

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let response = client.get_torrents(filters).await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let result = response.get_result("get_torrents")?;
        assert!(!result.is_empty());
        Ok(())
    }
}
