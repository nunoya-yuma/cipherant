use anyhow::Result;

pub trait LlmClient {
    fn complete(&self, prompt: &str) -> impl std::future::Future<Output = Result<String>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLlmClient;

    impl LlmClient for MockLlmClient {
        async fn complete(&self, prompt: &str) -> Result<String> {
            Ok(format!("LLM Response: {}", prompt))
        }
    }

    #[tokio::test]
    async fn test_send_query_to_llm() {
        let llm = MockLlmClient {};
        let response = llm.complete("test message").await.unwrap();

        assert_eq!(response, "LLM Response: test message");
    }
}
