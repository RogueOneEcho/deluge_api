use crate::client::{deserialize_response, get_random_u32};
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

impl DelugeClient {
    /// Add torrents by file
    ///
    /// # Warning
    /// Deluge will throw an exception and the API call will hang indefinitely
    /// if the torrent hash is already in the session.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn add_torrents(
        &mut self,
        torrents: Vec<TorrentPath>,
    ) -> Result<Response<Vec<Torrent>>, Error> {
        let method = "web.add_torrents";
        let data = json!({
            "method": method,
            "params": [ torrents ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentPath {
    pub path: String,
    pub options: Options,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Options {
    /// Directory to download the files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_location: Option<String>,
    /// File priority list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_priorities: Option<Vec<i32>>,
    /// Start the torrent paused
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_paused: Option<bool>,
    /// Maximum download speed in bytes per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_download_speed: Option<f64>,
    /// Maximum upload speed in bytes per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_speed: Option<f64>,
    /// Maximum number of connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    /// Move completed downloads to another directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_completed: Option<bool>,
    /// Path to move completed downloads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_completed_path: Option<String>,
    /// Add the torrent in seed mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_mode: Option<bool>,
    /// Download files sequentially
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_download: Option<bool>,
    /// Skip the hash check when adding the torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hash_check: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Torrent {
    added: bool,
    hash: String,
}

impl<'de> Deserialize<'de> for Torrent {
    /// Deserialize from `[ added, hash ]` format
    #[allow(clippy::absolute_paths)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: [Value; 2] = Deserialize::deserialize(deserializer)?;
        let Value::Bool(added) = array[0] else {
            return Err(serde::de::Error::custom("Expected a boolean for 'added'"));
        };
        let Value::String(hash) = array[1].clone() else {
            return Err(serde::de::Error::custom("Expected a string for 'hash'"));
        };
        Ok(Torrent { added, hash })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::get_test_options;
    use crate::DelugeClient;
    use log::trace;
    use rogue_logging::{Error, Logger};

    #[tokio::test]
    #[ignore]
    async fn add_torrents() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options = get_test_options()?;
        let mut client = DelugeClient::from_options(options.clone());
        let torrent = TorrentPath {
            path: "/srv/shared/tests/example-1.torrent".to_owned(),
            options: Options {
                download_location: Some("/srv/shared/tests".to_owned()),
                skip_hash_check: Some(true),
                ..Options::default()
            },
        };

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let response = client.add_torrents(vec![torrent]).await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let _result = response.get_result("add_torrents")?;
        Ok(())
    }
}
