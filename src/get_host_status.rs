use crate::client::{deserialize_response, get_random_u32};
use crate::{DelugeClient, Response};
use rogue_logging::Error;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;

impl DelugeClient {
    /// Get the current status for the specified host.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_host_status(&mut self, id: &str) -> Result<Response<Host>, Error> {
        let method = "web.get_host_status";
        let data = json!({
            "method": method,
            "params": [ id ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }
}

#[derive(Debug, Serialize)]
pub struct Host {
    id: String,
    status: Status,
    version: String,
}

impl<'de> Deserialize<'de> for Host {
    /// Deserialize from `[ id, status, version ]` format
    #[allow(clippy::absolute_paths)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: [String; 3] = Deserialize::deserialize(deserializer)?;
        let status: Status = serde_json::from_str(&format!(r#""{}""#, array[1]))
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(Host {
            id: array[0].clone(),
            status,
            version: array[2].clone(),
        })
    }
}

#[derive(Debug)]
pub enum Status {
    Online,
    Connected,
    Other(String),
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let status: String = Deserialize::deserialize(deserializer)?;
        Ok(match status.as_str() {
            "Online" => Status::Online,
            "Connected" => Status::Connected,
            other => Status::Other(other.to_owned()),
        })
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Status::Online => serializer.serialize_str("Online"),
            Status::Connected => serializer.serialize_str("Connected"),
            Status::Other(value) => serializer.serialize_str(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DelugeClient;
    use log::trace;
    use rogue_logging::{Error, Logger};

    use crate::options::get_test_options;

    #[tokio::test]
    async fn get_host_status() -> Result<(), Error> {
        // Arrange
        Logger::force_init("deluge_api".to_owned());
        let options = get_test_options()?;
        let mut client = DelugeClient::from_options(options);

        // Act
        let response = client.login().await?;
        trace!("{}", response.to_json_pretty());
        let response = client.get_hosts().await?;
        trace!("{}", response.to_json_pretty());
        let result = response.get_result("get_hosts")?;
        let id = result
            .first()
            .expect("should be at least one host")
            .id
            .clone();
        let response = client.get_host_status(&id).await?;
        trace!("{}", response.to_json_pretty());

        // Assert
        let _result = response.get_result("get_host_status")?;
        Ok(())
    }
}
