use anyhow::{Ok, Result};
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

/// Represents parsed content from a web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContent {
    /// The URL of the page
    pub url: String,
    /// The title of the page (if available)
    pub title: Option<String>,
    /// The main text content of the page
    pub text: String,
}

/// User-Agent string used for all HTTP requests
pub(crate) const USER_AGENT: &str = "cipherant/0.1.0";

/// Trait for HTTP client abstraction (enables mocking in tests)
pub(crate) trait HttpClient {
    async fn get(&self, url: &str) -> Result<String>;
}

pub(crate) struct ReqwestClient;

impl HttpClient for ReqwestClient {
    async fn get(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()?;
        let response = client.get(url).send().await?;
        let text = response.text().await?;

        Ok(text)
    }
}

pub async fn fetch_url(url: &str) -> Result<PageContent> {
    let request_client = ReqwestClient {};
    let page_content = fetch_url_with_client(&request_client, url).await?;

    Ok(page_content)
}

/// Fetch URL content using the provided HTTP client
async fn fetch_url_with_client<C: HttpClient>(client: &C, url: &str) -> Result<PageContent> {
    let html = client.get(url).await?;
    Ok(parse_html(url, &html))
}

fn parse_html(url: &str, html: &str) -> PageContent {
    let document = Html::parse_document(html);

    // Extract title
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>());

    // Extract <p> tag (body text)
    let p_selector = Selector::parse("p").unwrap();
    let body = document
        .select(&p_selector)
        .map(|element| element.text().collect())
        .collect::<Vec<String>>()
        .join("\n\n");

    PageContent {
        url: url.to_string(),
        title,
        text: body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_extracts_title() {
        // Arrange
        let html = r#"
            <html>
                <head><title>Test Page</title></head>
                <body><p>Hello World</p></body>
            </html>
        "#;

        // Act
        let result = parse_html("https://example.com", html);

        // Assert
        assert_eq!(result.title, Some("Test Page".to_string()));
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_parse_html_extracts_paragraphs() {
        // Multiple <p> tags should be joined with "\n\n"
        let html = r#"
            <html>
                <body>
                    <p>First paragraph</p>
                    <p>Second paragraph</p>
                </body>
            </html>
        "#;

        let result = parse_html("https://example.com", html);

        assert_eq!(result.text, "First paragraph\n\nSecond paragraph");
    }

    #[test]
    fn test_parse_html_handles_missing_title() {
        // HTML without <title> tag should return None
        let html = r#"
            <html>
                <body><p>Content without title</p></body>
            </html>
        "#;

        let result = parse_html("https://example.com", html);

        assert_eq!(result.title, None);
    }

    /// Mock HTTP client for testing
    struct MockHttpClient {
        response: String,
    }

    impl HttpClient for MockHttpClient {
        async fn get(&self, _url: &str) -> Result<String> {
            Ok(self.response.clone())
        }
    }

    #[tokio::test]
    async fn test_fetch_url_with_mock_client() {
        // Arrange: Create a mock client that returns fake HTML
        let mock_client = MockHttpClient {
            response: r#"<html><head><title>Mock Page</title></head><body><p>Mock content</p></body></html>"#.to_string(),
        };

        // Act
        let result = fetch_url_with_client(&mock_client, "https://example.com")
            .await
            .unwrap();

        assert_eq!(result.title, Some("Mock Page".to_string()));
        assert_eq!(result.text, "Mock content");
    }
}
