use clap::Parser;

use cipherant::agent::create_research_agent;
use cipherant::cli::Cli;
use rig::completion::Prompt;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let agent = create_research_agent();
    let response = match agent.prompt(args.prompt).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    println!("{}", response);
}
