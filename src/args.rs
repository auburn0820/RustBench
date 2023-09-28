use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {

    #[arg()]
    http_method: String,
    #[arg()]
    url: String,
    #[arg(short = 'r', long = "request")]
    request_count: u32,
    #[arg(short = 't', long = "thread")]
    thread_count: u32,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}