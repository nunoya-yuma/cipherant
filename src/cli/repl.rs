use futures::StreamExt;
use log::{error, warn};
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::message::{AssistantContent, Message, UserContent};
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;
use rig::OneOrMany;
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

    // Conversation history for multi-turn context
    let mut conversation_history: Vec<Message> = vec![];

    loop {
        let input = match rl.readline(PROMPT) {
            Ok(line) => line.trim().to_string(),
            Err(ReadlineError::Interrupted) => {
                /* Nothing to do */
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                warn!("Readline error: {}", err);
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

        let user_message = Message::User {
            content: OneOrMany::one(UserContent::text(&input)),
        };
        conversation_history.push(user_message);

        // Stream with conversation history
        let mut stream = agent
            .stream_chat(&input, conversation_history.clone())
            .await;

        let mut response_text = String::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => {
                    print!("{}", text.text);
                    response_text.push_str(&text.text);
                    io::stdout().flush().unwrap();
                }
                Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                    // Final response from LLM
                }
                Err(e) => {
                    error!("Stream error: {}", e);
                    break;
                }
                _ => {} // Others(tool call etc.)
            }
        }
        println!();

        let assistant_message = Message::Assistant {
            id: None,
            content: OneOrMany::one(AssistantContent::text(response_text)),
        };
        conversation_history.push(assistant_message);
    }

    // Save history for next session
    _ = rl.save_history(HISTORY_FILE);
}
