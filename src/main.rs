use std::thread;
use clap::Parser;
use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();
    let requests = args.request_count / args.thread_count;

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

