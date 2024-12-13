use crate::client::{deserialize_response, get_random_u32};
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde_json::json;

impl DelugeClient {
    /// Login and get a session cookie
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/devguide/how-to/curl-jsonrpc.html>
    pub async fn login(&mut self) -> Result<Response<bool>, Error> {
        let method = "auth.login";
        let data = json!({
            "method": method,
            "params": [ self.password ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}

#[cfg(test)]
mod tests {
    use crate::DelugeClient;
    use crate::DelugeClientOptions;
    use log::trace;
    use reqwest::cookie::CookieStore;
    use reqwest::Url;
    use rogue_config::{OptionsProvider, YamlOptionsProvider};
    use rogue_logging::{Error, Logger};

    #[tokio::test]
    async fn login() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options: DelugeClientOptions = YamlOptionsProvider::get()?;
        let mut client = DelugeClient::from_options(options);

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let result = response.get_result("login")?;
        assert!(result);
        let url = Url::parse(&client.api_url.clone()).expect("url should parse");
        let cookies = client.cookies.cookies(&url);
        trace!("{cookies:?}");
        assert!(cookies.is_some());
        Ok(())
    }
}
