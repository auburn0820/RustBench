use std::sync::Arc;

use anyhow::Error;
use clap::Parser;

use crate::args::Args;
use crate::http::send_request;

mod args;
mod http;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let method = Arc::new(args.http_method);
    let url = Arc::new(args.url);
    let data = Arc::new(args.data);
    let requests = args.request_count;

    let handles: Vec<_> = (0..requests)
        .map(|_| {
            let method_clone = Arc::clone(&method);
            let url_clone = Arc::clone(&url);
            let data_clone = Arc::clone(&data);
            tokio::spawn(async move {
                for _ in 0..requests {
                    let data_option = Arc::clone(&data_clone).as_ref().clone();
                    match send_request(method_clone.to_string(), url_clone.clone().to_string(), data_option).await {
                        Ok(response) => {
                            if let Err(e) = print_response(response).await {
                                eprintln!("Error printing response: {:?}", e);
                            }
                        }
                        Err(error) => { eprintln!("Error: {error}") }
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }
}

pub async fn print_response(response: reqwest::Response) -> anyhow::Result<()> {
    println!("Status: {}", response.status());
    for (key, value) in response.headers() {
        println!("{:?}: {:?}", key, value);
    }

    // Get the response text
    let text = response.text().await.map_err(Error::new)?;

    // Print the response text
    println!("{}", text);

    Ok(())
}

/*
Add HTTP request method

src/http.rs: HTTP 요청 메소드 프로토타입 작성
src/main.rs: `make_request()` 메소드 호출 프로토타입 코드 작성
Cargo.toml: reqwest, tokio, anyhow 라이브러리 의존성 추가
 */