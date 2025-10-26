mod cli;
mod collectors;

use clap::Parser;
use cli::Cli;

use collectors::web;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("Fetching: {}", args.url);

    match web::fetch_url(&args.url).await {
        Ok(text) => println!("{}", text),
        Err(e) => eprintln!("{}", e),
    }
}
