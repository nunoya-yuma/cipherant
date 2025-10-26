mod cli;
mod collectors;

use clap::Parser;
use cli::Cli;

use collectors::web;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("Fetching: {}", args.url);

    let content = match web::fetch_url(&args.url).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let title = content.title.as_deref().unwrap_or("(No title)");
    println!("---\nTitle: {}\n---", title);
    println!("{}\n---", content.text);
}
