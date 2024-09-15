use anyhow::{Context, Result};
use backoff::ExponentialBackoff;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const BASE_URL: &str = "https://api.coingecko.com/api/v3";
const CONCURRENT_REQUESTS: usize = 2;
const RATE_LIMIT_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug, Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
    current_price: Option<f64>,
}

async fn fetch_page(client: &Client, page: u32) -> Result<Vec<Coin>> {
    let url = format!(
        "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page={}&sparkline=false",
        BASE_URL, page
    );

    let response = backoff::future::retry(ExponentialBackoff::default(), || async {
        Ok(client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Coin>>()
            .await?)
    })
    .await
    .context("Failed to fetch page after retries")?;

    Ok(response)
}

async fn fetch_all_pages(client: &Client, total_pages: u32) -> Result<Vec<Coin>> {
    let results = stream::iter(1..=total_pages)
        .map(|page| {
            let client = client.clone();
            tokio::spawn(async move {
                tokio::time::sleep(RATE_LIMIT_DELAY).await;
                fetch_page(&client, page).await
            })
        })
        .buffer_unordered(CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await;

    let mut all_coins = Vec::new();
    for result in results {
        let coins = result.context("Task panicked")??;
        all_coins.extend(coins);
    }

    Ok(all_coins)
}
#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .user_agent("CoinGeckoRustClient/1.0")
        .build()
        .context("Failed to create HTTP client")?;

    let total_pages = 5; // Adjust this value based on your needs

    let coins = fetch_all_pages(&client, total_pages).await?;

    println!("Total coins fetched: {}", coins.len());
    for coin in coins.iter().take(10) {
        println!(
            "{} ({}): ${}",
            coin.name,
            coin.symbol,
            coin.current_price.unwrap_or(0.0)
        );
    }

    Ok(())
}
