use std::thread;
use anyhow::Error;
use clap::Parser;
use crate::args::Args;
use crate::http::make_request;

mod args;
mod http;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let requests = args.request_count / args.thread_count;


    match make_request(&*args.http_method, args.url).await {
        Ok(response) => {
            if let Err(e) = print_response(response).await {
                eprintln!("Error printing response: {:?}", e);
            }
        }
        Err(error) => { eprintln!("Error: {error}") }
    }

    let handles: Vec<_> = (0..args.thread_count)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..requests {
                    println!("Hello?");
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
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