use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct DelugeClientOptions {
    /// Deluge Web API host including port but without protocol or password
    ///
    /// # Examples
    /// - `localhost`
    /// - `example.com`
    /// - `example.com:3000`
    /// - `127.0.0.1`
    pub host: String,

    /// Deluge Web API password
    pub password: String,

    /// User agent
    pub user_agent: Option<String>,

    /// Number of requests permitted per `rate_limit_duration`
    pub rate_limit_count: Option<usize>,

    /// Duration before rate limit is reset
    pub rate_limit_duration: Option<usize>,
}
