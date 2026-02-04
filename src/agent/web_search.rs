use dotenvy::dotenv;
use log::{error, info};
use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

/// Arguments for the WebSearch tool
#[derive(Deserialize)]
pub struct WebSearchArgs {
    pub query: String,
}

/// A single search result
#[derive(Serialize, Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// Output from the WebSearch tool
#[derive(Serialize)]
pub struct WebSearchOutput {
    pub results: Vec<SearchResult>,
}

/// Error type for WebSearch tool
#[derive(Debug, thiserror::Error)]
pub enum WebSearchError {
    #[error("API key not found: set TAVILY_API_KEY environment variable")]
    ApiKeyNotFound,
    #[error("Search failed: {0}")]
    SearchError(String),
}

/// WebSearch tool for searching the web using Tavily API
pub struct WebSearch;

impl rig::tool::Tool for WebSearch {
    const NAME: &'static str = "web_search";
    type Error = WebSearchError;
    type Args = WebSearchArgs;
    type Output = WebSearchOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Searches the web for information using a query".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Searching for: {} ...", args.query);

        dotenv().ok();
        let tavily_api_key = match env::var("TAVILY_API_KEY") {
            Err(_e) => return Err(WebSearchError::ApiKeyNotFound),
            Ok(k) => k,
        };

        let client = reqwest::Client::new();
        const TAVILY_RESEARCH_URL: &str = "https://api.tavily.com/search";
        let body = json!(
            {
                "api_key": tavily_api_key,
                "query": args.query,
            }
        );
        let response = client.post(TAVILY_RESEARCH_URL).json(&body).send().await;

        let response_body = match response {
            Ok(r) => r.text().await.unwrap(),
            Err(e) => {
                error!("{}", e);
                return Err(WebSearchError::SearchError(e.to_string()));
            }
        };
        let parsed_response: serde_json::Value = serde_json::from_str(&response_body).unwrap();
        let contents: Vec<SearchResult> = parsed_response["results"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|item| SearchResult {
                title: item["title"].as_str().unwrap_or("").to_string(),
                url: item["url"].as_str().unwrap_or("").to_string(),
                snippet: item["content"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(WebSearchOutput { results: contents })
    }
}

#[cfg(test)]
mod tests {
    use rig::tool::Tool;

    use super::*;
    #[test]

    fn test_web_search_args_deserialize() {
        const JSON_ARGS: &str = r#"{"query": "How is the weather in Tokyo today?"}"#;
        let args: WebSearchArgs = serde_json::from_str(JSON_ARGS).unwrap();
        assert_eq!(args.query, "How is the weather in Tokyo today?");
    }

    #[test]
    fn test_web_search_output_serialize() {
        let output = WebSearchOutput {
            results: vec![
                SearchResult {
                    title: "Title1".to_string(),
                    url: "http://example.com1".to_string(),
                    snippet: "This is a example page1".to_string(),
                },
                SearchResult {
                    title: "Title2".to_string(),
                    url: "http://example.com2".to_string(),
                    snippet: "This is a example page2".to_string(),
                },
            ],
        };

        // Serialize to JSON string, then parse back to Value for comparison
        let json_str = serde_json::to_string(&output).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify structure using Value access
        assert_eq!(value["results"][0]["title"], "Title1");
        assert_eq!(value["results"][0]["url"], "http://example.com1");
        assert_eq!(value["results"][0]["snippet"], "This is a example page1");
        assert_eq!(value["results"][1]["title"], "Title2");
        assert_eq!(value["results"][1]["url"], "http://example.com2");
        assert_eq!(value["results"][1]["snippet"], "This is a example page2");
    }

    #[tokio::test]
    #[ignore]
    async fn test_call_web_search_tool() {
        const JSON_ARGS: &str = r#"{"query": "How is the weather in Tokyo today?"}"#;
        let args: WebSearchArgs = serde_json::from_str(JSON_ARGS).unwrap();

        let tool = WebSearch {};
        let response = tool.call(args).await.unwrap();
        assert!(!response.results[0].title.is_empty());
        assert!(!response.results[0].url.is_empty());
        assert!(!response.results[0].snippet.is_empty());
        assert!(!response.results[1].title.is_empty());
        assert!(!response.results[1].url.is_empty());
        assert!(!response.results[1].snippet.is_empty());
    }
}
