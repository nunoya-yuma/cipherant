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

pub async fn fetch_url(url: &str) -> Result<PageContent> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    Ok(parse_html(url, &text))
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
        title: title,
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
}
