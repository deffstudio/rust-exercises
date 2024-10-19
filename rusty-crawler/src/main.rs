mod cli;
mod crawler;
mod parser;
mod word_cloud;

use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse_args();
    let urls = crawler::crawl(&args.url, args.depth).await?;
    let words = parser::parse_urls(urls).await?;
    word_cloud::generate_word_cloud(words, &args.output)?;
    println!("Word cloud generated successfully!");
    Ok(())
}
