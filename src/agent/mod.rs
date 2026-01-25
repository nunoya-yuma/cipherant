mod web_fetch;

use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::providers::ollama;
pub use web_fetch::WebFetch;

pub fn create_research_agent() -> Agent<rig::providers::ollama::CompletionModel> {
    let client: ollama::Client = ollama::Client::builder()
        .api_key(Nothing)
        .build()
        .expect("Failed to create Ollama client");

    client
        .agent("qwen3")
        .preamble("You are a research assistant that helps users gather and summarize information from the web")
        .tool(WebFetch)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::completion::Prompt;

    #[tokio::test]
    #[ignore]
    async fn test_research_agent_with_web_fetch() {
        let agent = create_research_agent();
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }
}
