// src/lib.rs
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct HeadlineData {
    pub url: String,
    pub headline: String,
}

pub async fn scrape_url(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("h1").unwrap();

    document
        .select(&selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .context("No headline found")
}
