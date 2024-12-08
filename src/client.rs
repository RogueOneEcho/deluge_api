use std::sync::Arc;
#[cfg(test)]
use crate::{DelugeClientFactory, DelugeClientOptions};
use colored::Colorize;
use log::*;
use rand::Rng;
use reqwest::{Client, Response};
use rogue_logging::Error;
use serde_json::{json, Value};
use std::time::{Duration, SystemTime};
use reqwest::cookie::Jar;
use tower::limit::RateLimit;
use tower::ServiceExt;
use crate::schema::ApiResponse;

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
    pub async fn auth_login(&mut self) -> Result<ApiResponse, Error> {
        let method = "auth.login";
        let data = json!({
            "method": method,
            "params": [ self.password ],
            "id": get_random_u32()
        });
        let response = self.request(method, data).await?;
        let status_code = response.status();
        let json = response.text().await.map_err(|e| Error {
            action: method.to_owned(),
            message: e.to_string(),
            status_code: None,
            ..Error::default()
        })?;
        if status_code.is_success() {
            return serde_json::from_str::<ApiResponse>(&json).map_err(|e| Error {
                action: method.to_owned(),
                message: e.to_string(),
                status_code: None,
                ..Error::default()
            });
        }
        Err(Error {
            action: method.to_owned(),
            message: json,
            status_code: Some(status_code.as_u16()),
            ..Error::default()
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
            action: method.to_owned(),
            message: e.to_string(),
            status_code: None,
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
