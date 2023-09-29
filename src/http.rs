use anyhow::{Result, anyhow};
use reqwest::Response;

pub async fn make_request(method: &str, url: String) -> Result<Response> {
    let response = match method.to_uppercase().as_str() {
        "GET" => reqwest::get(url).await?,
        _ => return Err(anyhow!("Invalid HTTP method")),
    };

    Ok(response)
}