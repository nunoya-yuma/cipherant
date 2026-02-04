mod history;
mod repl;

use clap::Parser;

pub use history::{ConversationHistory, DEFAULT_MAX_TURNS};

pub use repl::run_interactive;

#[derive(Parser, Debug)]
#[command(name = "cipherant")]
#[command(about = "Personal Research Agent", long_about = None)]
pub struct Cli {
    /// Start interactive mode (REPL)
    #[arg(short, long)]
    pub interactive: bool,

    /// Query prompt (required if not in interactive mode)
    pub prompt: Option<String>,
}
