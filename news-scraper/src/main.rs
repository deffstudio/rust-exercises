use anyhow::{Context, Result};
use clap::Parser;
use news_scraper::{scrape_url, HeadlineData};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut results = Vec::new();

    for url in args.urls {
        match scrape_url(&url).await {
            Ok(headline) => results.push(HeadlineData {
                url: url.clone(),
                headline,
            }),
            Err(err) => eprintln!("Error scraping {}: {}", url, err),
        }
    }

    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("headlines.json", json).context("Failed to write headlines.json")?;

    println!("Scraped {} headlines", results.len());
    Ok(())
}
