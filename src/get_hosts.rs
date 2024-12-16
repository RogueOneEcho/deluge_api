use crate::client::{deserialize_response, get_random_u32};
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
impl DelugeClient {
    /// Get the hosts in the hostlist.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_hosts(&mut self) -> Result<Response<Vec<Host>>, Error> {
        let method = "web.get_hosts";
        let data = json!({
            "method": method,
            "params": [],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub id: String,
    #[allow(clippy::struct_field_names)]
    pub host: String,
    pub port: u16,
    pub user: String,
}

#[cfg(test)]
mod tests {
    use crate::{DelugeClient, DelugeClientOptions};
    use log::trace;
    use rogue_config::{OptionsProvider, YamlOptionsProvider};
    use rogue_logging::{Error, LoggerBuilder};

    #[tokio::test]
    async fn get_hosts() -> Result<(), Error> {
        // Arrange
        let _ = LoggerBuilder::new().create();
        let options: DelugeClientOptions = YamlOptionsProvider::get()?;
        let mut client = DelugeClient::from_options(options);

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let response = client.get_hosts().await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let result = response.get_result("get_hosts")?;
        assert!(!result.is_empty());
        Ok(())
    }
}
