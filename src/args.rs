use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {

    #[arg()]
    pub(crate) http_method: String,
    #[arg()]
    pub(crate) url: String,
    #[arg(short = 'r', long = "request")]
    pub(crate) request_count: u32,
    #[arg(short = 'd', long = "data")]
    pub(crate) data: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}