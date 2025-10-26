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
