use crate::options::get_test_options;
use crate::schema::Filters;
use crate::DelugeClient;
use log::trace;
use reqwest::cookie::CookieStore;
use reqwest::Url;
use rogue_logging::{Error, Logger};

#[tokio::test]
async fn login() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
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

#[tokio::test]
async fn get_hosts() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
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

#[tokio::test]
async fn get_torrent_status() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
    let mut client = DelugeClient::from_options(options.clone());

    // Act
    let response = client.login().await?;
    trace!("{}", response.to_json_pretty());
    let id = options
        .torrent_id
        .expect("example torrent_id should be set");
    let response = client.get_torrent_status(&id).await?;
    trace!("{}", response.to_json_pretty());

    // Assert
    let _result = response.get_result("get_torrent_status")?;
    Ok(())
}

#[tokio::test]
async fn get_interface() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
    let mut client = DelugeClient::from_options(options.clone());
    let filters = Filters {
        label: Some(vec!["linux".to_owned()]),
        ..Filters::default()
    };

    // Act
    let response = client.login().await?;
    trace!("{}", response.to_json_pretty());
    let response = client.get_interface(filters).await?;
    trace!("{}", response.to_json_pretty());

    // Assert
    let result = response.get_result("get_interface")?;
    assert!(result.torrents.is_some());
    Ok(())
}

#[tokio::test]
async fn get_torrents() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
    let mut client = DelugeClient::from_options(options.clone());
    let filters = Filters {
        label: Some(vec!["linux".to_owned()]),
        ..Filters::default()
    };

    // Act
    let response = client.login().await?;
    trace!("{}", response.to_json_pretty());
    let response = client.get_torrents(filters).await?;
    trace!("{}", response.to_json_pretty());

    // Assert
    let result = response.get_result("get_torrents")?;
    assert!(!result.is_empty());
    Ok(())
}
