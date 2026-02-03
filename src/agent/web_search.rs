use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
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
}
