use crate::handlers::handle_response;
use console::style;
use reqwest::blocking::Client;
use std::error::Error;

pub fn fetch_many_images(base_url: &str, _type: String, category: String) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}/{}", base_url, _type, category);
    println!(
        "[{}] Your request: {}", 
        style("INFO").green().bold().italic(),
        style(&url).cyan().bold().italic()
    );

    let client = Client::new();
    let req_body = "{}";
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("insomnia/8.4.4"));

    let response = client.post(&url).headers(headers).body(req_body).send()?;

    handle_response(response)?;
    Ok(())
}

pub fn fetch_single_image(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;

    handle_response(response)?;

    Ok(())
}
