use crate::Response;
#[cfg(test)]
use crate::{DelugeClientFactory, DelugeClientOptions};
use colored::Colorize;
use log::*;
use rand::Rng;
use reqwest::cookie::Jar;
use reqwest::Client;
use rogue_logging::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;
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

    pub(crate) async fn request(
        &mut self,
        method: &str,
        data: Value,
    ) -> Result<reqwest::Response, Error> {
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

pub(crate) fn get_random_u32() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub(crate) async fn deserialize_response<T: DeserializeOwned>(
    method: &str,
    response: reqwest::Response,
) -> Result<Response<T>, Error> {
    let status_code = Some(response.status().as_u16());
    let json = response.text().await.map_err(|e| Error {
        action: format!("get response body of {method} request"),
        domain: Some("Deluge API".to_owned()),
        message: e.to_string(),
        status_code,
        ..Error::default()
    })?;
    match serde_json::from_str::<Response<T>>(&json) {
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
