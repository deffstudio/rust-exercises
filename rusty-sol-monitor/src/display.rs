use crate::csv_writer::TokenCreationEvent;
use crate::metadata::TokenMetadata;
use chrono::{DateTime, Utc};
use colored::*;

// Display token creation information in the console.
pub fn display_token_creation_info(
    metadata: &TokenMetadata,
    block_time: Option<i64>,
) -> Option<TokenCreationEvent> {
    let name_colored = metadata.name.blue().bold();
    let symbol_colored = metadata.symbol.magenta().bold();
    let url_colored = metadata.url.green().bold();

    if let Some(time) = block_time {
        let time_formatted = DateTime::<Utc>::from_timestamp(time, 0)?.naive_utc();
        println!(
            "{} New Token Created! Name: {}, Symbol: {}, URL: {}, Block Time: {}",
            "[INFO]".blue().bold(),
            name_colored,
            symbol_colored,
            url_colored,
            time_formatted
        );
        return Some(TokenCreationEvent {
            mint_pubkey: metadata.name.clone(),
            name: metadata.name.clone(),
            symbol: metadata.symbol.clone(),
            url: metadata.url.clone(),
            block_time: time_formatted.to_string(),
        });
    }
    None
}
