use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

pub async fn crawl(start_url: &str, max_depth: u32) -> Result<Vec<String>> {
    let client = Client::new();
    let mut to_visit = vec![(start_url.to_string(), 0)];
    let mut visited = HashSet::new();
    let mut collected_urls = Vec::new();

    while let Some((url, depth)) = to_visit.pop() {
        if depth > max_depth || !visited.insert(url.clone()) {
            continue;
        }

        match client.get(&url).send().await {
            Ok(response) => {
                let body = response.text().await?;
                collected_urls.push(url.clone());

                if depth < max_depth {
                    let document = Html::parse_document(&body);
                    let selector = Selector::parse("a").unwrap();

                    for element in document.select(&selector) {
                        if let Some(href) = element.value().attr("href") {
                            if let Ok(absolute_url) = Url::parse(&url)?.join(href) {
                                to_visit.push((absolute_url.into(), depth + 1));
                            }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
    Ok(collected_urls)
}
