use dotenv::dotenv;
use log::info;
use reqwest::Client;
use std::env;
use std::error::Error;
use teloxide::dispatching::Dispatcher;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Me};
use teloxide::utils::command::BotCommands;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Help,
    Menu,
    Crypto,
}

async fn handle_message(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        // Match the command based on the text message
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                // Respond only with the Start command message
                bot.send_message(msg.chat.id, "Welcome! Use /menu to see the options.")
                    .await?;
            }
            Ok(Command::Help) => {
                // Respond only with the Help command message
                bot.send_message(msg.chat.id, "Available commands:\n/start - Start the bot\n/help - Show help\n/menu - Show menu")
                    .await?;
            }
            Ok(Command::Menu) => {
                // Respond only with the Menu command message
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("🪙 CoinGecko", "crypto"),
                        InlineKeyboardButton::callback("Option 2", "option2"),
                    ],
                    vec![
                        InlineKeyboardButton::callback("Option 3", "option3"),
                        InlineKeyboardButton::callback("Option 4", "option4"),
                        InlineKeyboardButton::callback("More info", "info"),
                    ],
                ]);

                bot.send_message(msg.chat.id, "Here is the menu:")
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(Command::Crypto) => {
                let crypto_prices = fetch_crypto_prices().await?;
                bot.send_message(msg.chat.id, crypto_prices).await?;
            }
            Err(_) => {
                // Handle unknown commands or invalid input
                bot.send_message(
                    msg.chat.id,
                    "Invalid command. Use /help to see the available options.",
                )
                .await?;
            }
        }
    }

    Ok(())
}

async fn handle_callback_query(
    bot: Bot,
    callback_query: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Callback query: {:?}", callback_query);

    // Acknowledge the callback query
    bot.answer_callback_query(callback_query.id).await?;

    let data = callback_query.data.unwrap_or_default();
    let response = match data.as_str() {
        "crypto" => fetch_crypto_prices()
            .await
            .unwrap_or("Failed to fetch crypto prices".to_string()),
        "option2" => "You chose Option 2!".to_string(),
        "option3" => "You chose Option 3!".to_string(),
        "option4" => "You chose Option 4!".to_string(),
        "info" => "Here is more info about the menu options!".to_string(),
        _ => "Unknown option".to_string(),
    };

    bot.send_message(callback_query.from.id, response).await?;
    Ok(())
}

async fn fetch_crypto_prices() -> Result<String, Box<dyn Error + Send + Sync>> {
    // Fetch base URL from environment variable
    let base_url = env::var("COINGECKO_BASE_URL")?;
    let client = Client::new();
    let url = format!(
        "{}simple/price?ids=bitcoin,ethereum,tether,bnb,solana&vs_currencies=usd",
        base_url
    );
    let response = client.get(&url).send().await?;
    let prices: serde_json::Value = response.json().await?;

    let bitcoin_price = prices["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
    let ethereum_price = prices["ethereum"]["usd"].as_f64().unwrap_or(0.0);
    let tether_price = prices["tether"]["usd"].as_f64().unwrap_or(0.0);
    //let bnb_price = prices["bnb"]["usd"].as_f64().unwrap_or(0.0);
    let solana_price = prices["solana"]["usd"].as_f64().unwrap_or(0.0);

    Ok(format!(
        "Bitcoin: ${:.2}\nEthereum: ${:.2}\nTether: ${:.2}\nSolana: ${:.2}",
        bitcoin_price, ethereum_price, tether_price, solana_price,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Starting command bot...");

    let bot_token = env::var("TELOXIDE_TOKEN").expect("Cannot get the TELOXIDE_TOKEN env variable");
    let bot = Bot::new(bot_token);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback_query));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
