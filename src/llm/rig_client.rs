use anyhow::{Ok, Result};
use rig::{
    completion::{message::AssistantContent, CompletionModel, CompletionRequest},
    providers::ollama,
};

use super::LlmClient;

/// RigClient wraps Rig library to implement LlmClient trait
pub struct RigClient {
    model: String,
}

impl RigClient {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}

impl LlmClient for RigClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        // Send query
        let client = ollama::Client::new();
        let comp_model = client.completion_model(&self.model);
        let req = CompletionRequest {
            prompt: rig::message::Message::User {
                content: rig::one_or_many::OneOrMany::one(rig::message::UserContent::text(prompt)),
            },
            preamble: None,
            chat_history: vec![],
            documents: vec![],
            tools: vec![],
            temperature: Some(0.7),
            max_tokens: None,
            additional_params: None,
        };

        // Parse response
        let llm_response = comp_model.completion(req).await?;
        let response_contents = llm_response
            .choice
            .iter()
            .filter_map(|c| match c {
                AssistantContent::Text(text) => Some(text.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(response_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_rig_client_with_ollama() {
        let client = RigClient::new("llama3.2");
        let response = client.complete("Say hello").await.unwrap();

        assert!(!response.is_empty());
    }
}
