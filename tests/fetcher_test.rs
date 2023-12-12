use httpmock::MockServer;
use std::error::Error;
use waifu4me::fetcher::{fetch_many_images, fetch_single_image};

#[test]
fn test_fetch_many_images_success() -> Result<(), Box<dyn Error>> {
    // Start a mock server
    let server = MockServer::start();

    // Set up the mock response
    let mock_url = server.base_url();
    let mock_response_body = r#"{"files": ["https://i.waifu.pics/test.jpg"]}"#;
    server.mock(|when, then| {
        when.method("POST").path("/type/category");
        then.status(200).body(mock_response_body);
    });

    // Call the fetch_many_images function with the mock server URL
    let base_url = &mock_url;
    let _type = "type".to_string();
    let category = "category".to_string();

    let result = fetch_many_images(base_url, _type, category);

    // Assert that the function returned Ok
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_fetch_single_image_success() -> Result<(), Box<dyn Error>> {
    // Start a mock server
    let server = MockServer::start();

    // Set up the mock response
    let mock_url = server.base_url();
    let mock_response_body = r#"{"url": "https://i.waifu.pics/test.jpg"}"#;
    server.mock(|when, then| {
        when.method("GET").path("/");
        then.status(200).body(mock_response_body);
    });

    // Call the fetch_single_image function with the mock server URL
    let url = &mock_url;
    let result = fetch_single_image(url);

    // Assert that the function returned Ok
    assert!(result.is_ok());

    Ok(())
}
