use anyhow::{Context, Error, Result};
use futures::future::try_join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub async fn parse_urls(urls: Vec<String>) -> Result<HashMap<String, usize>> {
    let client = Client::new();
    let futures = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let response = client
                .get(&url)
                .send()
                .await
                .context("Failed to send request")?
                .text()
                .await
                .context("Failed to get response text")?;
            Ok::<HashMap<std::string::String, usize>, Error>(parse_content(&response))
        }
    });

    let results = try_join_all(futures).await?;

    let mut word_counts = HashMap::new();
    for result in results {
        for (word, count) in result {
            *word_counts.entry(word).or_insert(0) += count;
        }
    }

    Ok(word_counts)
}

fn parse_content(content: &str) -> HashMap<String, usize> {
    let document = Html::parse_document(content);
    let selector = Selector::parse("body").unwrap();
    let body = document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>());

    if let Some(text) = body {
        text.split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| word.len() > 3)
            .fold(HashMap::new(), |mut acc, word| {
                *acc.entry(word).or_insert(0) += 1;
                acc
            })
    } else {
        HashMap::new()
    }
}
