#[cfg(test)]
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::BufReader;
#[cfg(test)]
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize)]
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

    /// Torrent id
    pub torrent_id: Option<String>,
}

#[cfg(test)]
fn from_yaml_file(path: &Path) -> Result<DelugeClientOptions, Error> {
    let file = File::open(path).map_err(|e| Error {
        action: "open options file".to_owned(),
        message: e.to_string(),
        domain: Some("file system".to_owned()),
        ..Error::default()
    })?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).map_err(|e| Error {
        action: "deserialize options file".to_owned(),
        message: e.to_string(),
        domain: Some("deserialization".to_owned()),
        ..Error::default()
    })
}

#[cfg(test)]
pub(crate) fn get_test_options() -> Result<DelugeClientOptions, Error> {
    from_yaml_file(&PathBuf::from("config.yml"))
}
