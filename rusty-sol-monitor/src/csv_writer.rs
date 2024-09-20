use csv::Writer;
use std::fs::OpenOptions;

// Struct for storing token creation events
#[derive(serde::Serialize)]
pub struct TokenCreationEvent {
    pub mint_pubkey: String,
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub block_time: String,
}

// Write token creation events to CSV
pub fn write_to_csv(event: &TokenCreationEvent) {
    let file_path = "token_creation_events.csv";

    // Open or create csv file in the append mode
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open CSV file");

    let mut wtr = Writer::from_writer(file);

    // Write the event to CSV
    wtr.serialize(event).expect("Failed to write CSV");
    wtr.flush().expect("Failed to flush CSV writer");
}
