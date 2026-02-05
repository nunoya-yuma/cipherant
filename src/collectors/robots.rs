use std::collections::HashMap;

use anyhow::Result;
use texting_robots::Robot;

use super::web::{HttpClient, USER_AGENT};

/// Cache for robots.txt per domain origin.
/// Stores parsed `Robot` instances keyed by origin (e.g. "https://example.com").
pub(crate) struct RobotsCache {
    cache: HashMap<String, Option<Robot>>,
}

impl RobotsCache {
    pub(crate) fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Check if the given URL is allowed by the site's robots.txt.
    /// Returns `true` (allow) on fetch/parse errors (graceful fallback).
    pub(crate) async fn is_allowed<C: HttpClient>(&mut self, client: &C, url: &str) -> bool {
        // TODO(human): Implement this method
        // 1. Extract the origin from the URL (e.g. "https://example.com")
        // 2. Check if the origin is already cached
        // 3. If not cached, fetch robots.txt and parse it
        // 4. Use the Robot to check if the URL is allowed for USER_AGENT
        // 5. Return true on any error (graceful fallback)
        todo!()
    }
}

/// Extract the origin (scheme + host + port) from a URL.
/// e.g. "https://example.com/path" -> "https://example.com"
pub(crate) fn extract_origin(url: &str) -> Option<String> {
    // TODO(human): Implement this function
    // Parse the URL and return scheme + host (+ port if non-default)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Ok;

    struct MockHttpClient {
        responses: HashMap<String, String>,
    }

    impl MockHttpClient {
        fn new() -> Self {
            Self {
                responses: HashMap::new(),
            }
        }

        fn with_response(mut self, url: &str, body: &str) -> Self {
            self.responses.insert(url.to_string(), body.to_string());
            self
        }
    }

    impl HttpClient for MockHttpClient {
        async fn get(&self, url: &str) -> Result<String> {
            self.responses
                .get(url)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No mock response for {}", url))
        }
    }

    // --- extract_origin tests ---

    #[test]
    fn test_extract_origin_https() {
        assert_eq!(
            extract_origin("https://example.com/path/to/page"),
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_extract_origin_with_port() {
        assert_eq!(
            extract_origin("http://localhost:8080/api"),
            Some("http://localhost:8080".to_string())
        );
    }

    #[test]
    fn test_extract_origin_invalid_url() {
        assert_eq!(extract_origin("not-a-url"), None);
    }

    // --- is_allowed tests ---

    #[tokio::test]
    async fn test_allowed_when_robots_txt_permits() {
        let client = MockHttpClient::new().with_response(
            "https://example.com/robots.txt",
            "User-agent: *\nAllow: /",
        );
        let mut cache = RobotsCache::new();

        assert!(cache.is_allowed(&client, "https://example.com/page").await);
    }

    #[tokio::test]
    async fn test_blocked_when_robots_txt_disallows() {
        let client = MockHttpClient::new().with_response(
            "https://example.com/robots.txt",
            "User-agent: *\nDisallow: /secret",
        );
        let mut cache = RobotsCache::new();

        assert!(
            !cache
                .is_allowed(&client, "https://example.com/secret/page")
                .await
        );
    }

    #[tokio::test]
    async fn test_allowed_when_robots_txt_fetch_fails() {
        // No mock response for robots.txt => fetch fails => graceful fallback to allow
        let client = MockHttpClient::new();
        let mut cache = RobotsCache::new();

        assert!(cache.is_allowed(&client, "https://example.com/page").await);
    }

    #[tokio::test]
    async fn test_cache_reuses_robots_txt_for_same_origin() {
        let client = MockHttpClient::new().with_response(
            "https://example.com/robots.txt",
            "User-agent: *\nDisallow: /blocked",
        );
        let mut cache = RobotsCache::new();

        // First call fetches robots.txt
        assert!(cache.is_allowed(&client, "https://example.com/ok").await);
        // Second call should reuse cached robots.txt (same origin)
        assert!(
            !cache
                .is_allowed(&client, "https://example.com/blocked/page")
                .await
        );
        // Only one entry in cache
        assert_eq!(cache.cache.len(), 1);
    }
}
