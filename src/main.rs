mod cli;
mod collectors;
mod llm;

use clap::Parser;
use cli::Cli;

use collectors::web;

use crate::llm::LlmClient;
use crate::llm::RigClient;

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

    const LLM_MODEL: &str = "llama3.2";
    let llm_client = RigClient::new(LLM_MODEL);
    let prompt: String = format!("Summarize this: {}", content.text);

    let llm_response = match llm_client.complete(prompt.as_str()).await {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("\n---LLM Summary\n{}\n---", llm_response);
}
