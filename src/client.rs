use crate::schema::{ApiResponse, Filters, Host, HostStatus, Interface, Torrent, TorrentFull};
#[cfg(test)]
use crate::{DelugeClientFactory, DelugeClientOptions};
use colored::Colorize;
use log::*;
use rand::Rng;
use reqwest::cookie::Jar;
use reqwest::{Client, Response};
use rogue_logging::Error;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tower::limit::RateLimit;
use tower::ServiceExt;

/// A client for the Deluge API
///
/// Created by an [`DelugeClientFactory`]
pub struct DelugeClient {
    pub api_url: String,
    pub password: String,
    pub cookies: Arc<Jar>,
    pub client: RateLimit<Client>,
}

impl DelugeClient {
    #[cfg(test)]
    pub(crate) fn from_options(options: DelugeClientOptions) -> DelugeClient {
        let factory = DelugeClientFactory { options };
        factory.create()
    }

    /// Login and get a session cookie
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/devguide/how-to/curl-jsonrpc.html>
    pub async fn login(&mut self) -> Result<ApiResponse<bool>, Error> {
        let method = "auth.login";
        let data = json!({
            "method": method,
            "params": [ self.password ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }

    /// Get the hosts in the hostlist.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_hosts(&mut self) -> Result<ApiResponse<Vec<Host>>, Error> {
        let method = "web.get_hosts";
        let data = json!({
            "method": method,
            "params": [],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }

    /// Get the current status for the specified host.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_host_status(&mut self, id: &str) -> Result<ApiResponse<HostStatus>, Error> {
        let method = "web.get_host_status";
        let data = json!({
            "method": method,
            "params": [ id ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }

    /// Get the status for a torrent, filtered by status keys.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_torrent_status(
        &mut self,
        id: &str,
    ) -> Result<ApiResponse<TorrentFull>, Error> {
        let method = "web.get_torrent_status";
        let data = json!({
            "method": method,
            "params": [ id, [] ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        deserialize_response(method, response).await
    }

    /// Gather the information required for updating the web interface.
    ///
    /// # See Also
    /// - <https://deluge.readthedocs.io/en/latest/reference/webapi.html>
    pub async fn get_interface(
        &mut self,
        filters: Filters,
    ) -> Result<ApiResponse<Interface>, Error> {
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

    /// Get all torrents matching the filter
    ///
    /// This is a wrapper for `get_interface()`
    pub async fn get_torrents(
        &mut self,
        filters: Filters,
    ) -> Result<ApiResponse<HashMap<String, Torrent>>, Error> {
        let method = "web.update_ui";
        let data = json!({
            "method": method,
            "params": [ [], filters ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        let response = deserialize_response::<Interface>(method, response).await?;
        Ok(ApiResponse {
            status_code: response.status_code,
            result: response.result.map(|x| x.torrents.unwrap_or_default()),
            error: response.error,
            id: response.id,
        })
    }

    async fn request(&mut self, method: &str, data: Value) -> Result<Response, Error> {
        trace!("{} request {method}", "Sending".bold());
        let api_url = self.api_url.clone();
        let client = self.wait_for_client().await;
        let start = SystemTime::now();
        let result = client.post(api_url).json(&data).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("{} response after {elapsed:.3}", "Received".bold());
        result.map_err(|e| Error {
            action: format!("send {method} request"),
            domain: Some("Deluge API".to_owned()),
            message: e.to_string(),
            ..Error::default()
        })
    }

    async fn wait_for_client(&mut self) -> &Client {
        let start = SystemTime::now();
        let client = self
            .client
            .ready()
            .await
            .expect("client should be available")
            .get_ref();
        let duration = start.elapsed().expect("duration should not fail");
        if duration > Duration::from_millis(200) {
            trace!(
                "{} {:.3} for rate limiter",
                "Waited".bold(),
                duration.as_secs_f64()
            );
        }
        client
    }
}

fn get_random_u32() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

async fn deserialize_response<T: DeserializeOwned>(
    method: &str,
    response: Response,
) -> Result<ApiResponse<T>, Error> {
    let status_code = Some(response.status().as_u16());
    let json = response.text().await.map_err(|e| Error {
        action: format!("get response body of {method} request"),
        domain: Some("Deluge API".to_owned()),
        message: e.to_string(),
        status_code,
        ..Error::default()
    })?;
    match serde_json::from_str::<ApiResponse<T>>(&json) {
        Ok(mut response) => {
            response.status_code = status_code;
            Ok(response)
        }
        Err(e) => {
            trace!("{json}");
            Err(Error {
                action: format!("deserialize response of Deluge API {method} request"),
                domain: Some("deserialization".to_owned()),
                message: e.to_string(),
                status_code,
                ..Error::default()
            })
        }
    }
}
