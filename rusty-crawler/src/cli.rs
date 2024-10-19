use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Starting url to crawl
    #[arg(short, long)]
    pub url: String,

    // Dept of crawling
    #[arg(short, long, default_value_t = 2)]
    pub depth: u32,

    /// Output file for the word cloud
    #[arg(short, long, default_value = "word_cloud.png")]
    pub output: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
