use std::io::{stdout, Write};
use std::sync::Arc;

use anyhow::Error;
use clap::Parser;
use futures::{stream, StreamExt};
use reqwest::Client;
use tokio::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand
};

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

    let start = Instant::now();
    let client = Client::new();

    let success_count = Arc::new(AtomicUsize::new(0));

    stream::iter(0..requests).map(|_| {
        let method_clone = Arc::clone(&method);
        let url_clone = Arc::clone(&url);
        let data_clone = Arc::clone(&data);
        let client_clone = client.clone();
        let count_clone = Arc::clone(&success_count);

        async move {
            let data_option = Arc::clone(&data_clone).as_ref().clone();
            match send_request(&client_clone, method_clone.to_string(), url_clone.clone().to_string(), data_option).await {
                Ok(_) => {
                    let count = count_clone.fetch_add(1, Ordering::Relaxed);
                    stdout().execute(cursor::MoveTo(0, 0)).unwrap()
                        .execute(terminal::Clear(ClearType::CurrentLine)).unwrap()
                        .execute(cursor::MoveTo(0, 0)).unwrap();
                    println!("Completed requests: {}", count + 1);
                    stdout().flush().unwrap();
                }
                Err(error) => {
                    eprintln!("Error: {}", error)
                }
            }
        }
    }).buffer_unordered(1_000).collect::<Vec<_>>().await;

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration)
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