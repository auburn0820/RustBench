use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use reqwest::{Client, Response};
use serde_json::{from_str, Value};

use HttpMethod::*;

pub async fn send_request(client: &Client, method: String, url: String, data: Option<String>) -> Result<Response> {
    let method = HttpMethod::from_str(method.as_str()).unwrap();
    let data = match &data {
        Some(json_str) => {
            match from_str(json_str) {
                Ok(json_value) => Some(json_value),
                Err(e) => return Err(anyhow!(e))
            }
        },
        None => None,
    };

    let response = match method {
        GET => build_request(client.get(url), data).await?,
        POST => build_request(client.post(url), data).await?,
        PUT => build_request(client.put(url), data).await?,
        PATCH => build_request(client.patch(url), data).await?,
        DELETE => client.delete(url).send().await?
    };

    Ok(response)
}

async fn build_request(builder: reqwest::RequestBuilder, data: Option<Value>) -> Result<Response, reqwest::Error> {
    match data {
        Some(data) => builder.json(&data).send().await,
        None => builder.send().await
    }
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH
}

impl FromStr for HttpMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "GET" => Ok(GET),
            "POST" => Ok(POST),
            "PUT" => Ok(PUT),
            "DELETE" => Ok(DELETE),
            "PATCH" => Ok(PATCH),
            _ => Err("No match"),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GET => "GET",
            POST => "POST",
            PUT => "PUT",
            DELETE => "DELETE",
            PATCH => "PATCH"
        })
    }
}