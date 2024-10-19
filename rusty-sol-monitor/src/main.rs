mod csv_writer;
mod display;
mod metadata;

use std::str::FromStr;
use std::sync::Arc;

use chrono::NaiveDateTime;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_sdk::signature::Signature;
use ws::{connect, Message};

use csv_writer::write_to_csv;
use display::display_token_creation_info;
use metadata::{get_mint_pubkey, get_token_metadata};

const SOLANA_WS_URL: &str =
    "wss://mainnet.helius-rpc.com/?api-key=03901544-4160-4757-8a44-3140c0cfb678";

#[tokio::main]
async fn main() {
    let client = Arc::new(RpcClient::new(
        "https://mainnet.helius-rpc.com/?api-key=03901544-4160-4757-8a44-3140c0cfb678s",
    ));

    let start_date =
        NaiveDateTime::parse_from_str("2023-09-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end_date =
        NaiveDateTime::parse_from_str("2023-09-30 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();

    loop {
        println!("Connectiong to Websocket...");
        connect(SOLANA_WS_URL, |out| {
            let client = Arc::clone(&client);
            move |msg: Message| {
                // Deserialize message into RpcConfirmedTransactionStatusWithSignature
                let transaction_status: RpcConfirmedTransactionStatusWithSignature =
                    serde_json::from_str(&msg.to_string()).unwrap();

                // Extract the signature
                let signature = Signature::from_str(&transaction_status.signature).unwrap();

                // Fetch mint pubkey from the transaction
                if let Some(mint_pubkey) = get_mint_pubkey(&client, &signature) {
                    // Fetch token metadata (name, symbol, url)
                    if let Some(metadata) = get_token_metadata(&client, &mint_pubkey) {
                        let block_time = transaction_status.block_time;
                        let event_info = display_token_creation_info(&metadata, block_time);

                        // Save event to CSV
                        if let Some(event) = event_info {
                            write_to_csv(&event);
                        }
                    }
                }
                out.send(Message::text("Keep listening")).unwrap();
                Ok(())
            }
        })
        .unwrap();

        // Delay before trying to reconnect if the connection closes
        println!("Websocket connection closed. Reconnecting in 5 seconds...");
    }
}
