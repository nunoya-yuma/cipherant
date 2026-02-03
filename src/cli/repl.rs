use futures::StreamExt;
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingPrompt;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self, Write};

const PROMPT: &str = "> ";
const HISTORY_FILE: &str = ".cipherant_history";

pub async fn run_interactive(agent: Agent<rig::providers::ollama::CompletionModel>) {
    println!("Cipherant Interactive Mode");
    println!("Type 'exit' or 'quit' to exit, Ctrl+D to quit\n");

    let mut rl = DefaultEditor::new().expect("Failed to create editor");

    // Load history from previous sessions
    _ = rl.load_history(HISTORY_FILE);

    loop {
        let input = match rl.readline(PROMPT) {
            Ok(line) => line.trim().to_string(),
            Err(ReadlineError::Interrupted) => {
                /* Nothing to do */
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        };

        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            break;
        }

        // Add input to history
        _ = rl.add_history_entry(&input);

        let mut stream = agent.stream_prompt(&input).await;

        while let Some(result) = stream.next().await {
            match result {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => {
                    print!("{}", text.text);
                    io::stdout().flush().unwrap();
                }
                Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                    // Final response from LLM
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
                _ => {} // Others(tool call etc.)
            }
        }
        println!();
    }

    // Save history for next session
    _ = rl.save_history(HISTORY_FILE);
}
