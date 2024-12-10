use std::sync::Arc;
use std::time::Duration;

use crate::DelugeClient;
use crate::DelugeClientOptions;
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, ClientBuilder};

/// The number of requests allowed per duration
const DEFAULT_RATE_COUNT: usize = 10;
const DEFAULT_RATE_DURATION: usize = 10;

/// Create a [`DelugeClient`]
pub struct DelugeClientFactory {
    pub options: DelugeClientOptions,
}

impl DelugeClientFactory {
    #[must_use]
    #[allow(clippy::as_conversions)]
    pub fn create(&self) -> DelugeClient {
        let rate_count = self.options.rate_limit_count.unwrap_or(DEFAULT_RATE_COUNT) as u64;
        let rate_duration = self
            .options
            .rate_limit_duration
            .unwrap_or(DEFAULT_RATE_DURATION) as u64;
        let rate_duration = Duration::from_secs(rate_duration);
        let cookies = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .default_headers(self.get_headers())
            .cookie_provider(cookies.clone())
            .build()
            .expect("Client builder should not fail");
        let client = tower::ServiceBuilder::new()
            .rate_limit(rate_count, rate_duration)
            .service(client);
        DelugeClient {
            api_url: format!("{}/json", self.options.host),
            cookies,
            password: self.options.password.clone(),
            client,
        }
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(user_agent) = self.options.user_agent.clone() {
            headers.insert(
                header::USER_AGENT,
                HeaderValue::try_from(user_agent).expect("user agent should not fail"),
            );
        }
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers
    }
}
