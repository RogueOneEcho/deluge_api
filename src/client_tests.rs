use reqwest::cookie::CookieStore;
use reqwest::Url;
use crate::options::get_test_options;
use crate::DelugeClient;
use rogue_logging::{Error, Logger};

#[tokio::test]
async fn auth_login() -> Result<(), Error> {
    // Arrange
    Logger::force_init("deluge_api".to_owned());
    let options = get_test_options()?;
    let mut client = DelugeClient::from_options(options);

    // Act
    let response = client.auth_login().await?;
    println!("{response}");

    // Assert
    assert!(response.result);
    assert_eq!(response.error, None);
    let url = Url::parse(&client.api_url.clone()).expect("url should parse");
    let cookies = client.cookies.cookies(&url);
    println!("{cookies:?}");
    assert!(cookies.is_some());
    Ok(())
}
