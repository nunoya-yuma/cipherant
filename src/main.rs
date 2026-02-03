use clap::Parser;

use cipherant::agent::create_research_agent;
use cipherant::cli::{run_interactive, Cli};
use rig::completion::Prompt;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let agent = create_research_agent();

    if args.interactive {
        run_interactive(agent).await;
    } else {
        // One-shot mode: require prompt argument
        let prompt = match args.prompt {
            Some(p) => p,
            None => {
                eprintln!("Error: prompt required. Use -i for interactive mode.");
                return;
            }
        };

        let response = match agent.prompt(prompt).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        println!("{}", response);
    }
}
