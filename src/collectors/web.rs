use anyhow::Result;
use reqwest;

pub async fn fetch_url(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}
