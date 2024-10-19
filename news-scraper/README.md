# Rust Web Scraper Exercise

## Scenario

You've been hired by a local news aggregator startup to create a web scraper that collects headlines from various news websites. They want a command-line tool that can scrape headlines from a list of URLs and save them to a JSON file.

## Requirements

1. Create a command-line application that accepts a list of URLs as arguments.
2. For each URL, scrape the main headline of the page.
3. Store the results in a JSON file with the following structure:
   ```json
   [
     {
       "url": "https://example.com",
       "headline": "Example Headline"
     },
     ...
   ]
   ```
4. Handle errors gracefully (e.g., network issues, parsing problems).
5. Implement concurrent scraping to improve performance.

## Tips

- Use the `reqwest` crate for making HTTP requests.
- Use the `scraper` crate for parsing HTML and extracting data.
- Use `serde` and `serde_json` for JSON serialization.
- Consider using `tokio` for asynchronous programming.
- Use `clap` or `structopt` for parsing command-line arguments.

## Bonus Challenges

1. Add a timeout for each request to handle slow-loading sites.
2. Implement a simple caching mechanism to avoid re-scraping recently visited sites.
3. Add an option to periodically scrape and update the JSON file, running as a daemon.

## Example Usage

```
$ ./news_scraper https://news.ycombinator.com https://reddit.com/r/rust
```

This should create a `headlines.json` file with the scraped data.

## Learning Outcomes

- Working with external crates in Rust
- Handling HTTP requests and HTML parsing
- Error handling in Rust
- Concurrent programming
- CLI argument parsing
- File I/O operations
- JSON serialization/deserialization
