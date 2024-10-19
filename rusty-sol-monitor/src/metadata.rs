use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiInstruction, UiParsedInstruction,
    UiTransactionEncoding,
};
use std::str::FromStr;

// Metaplex Token Metadata Program ID
const METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

// Struct to store metadata
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub url: String,
}

// Function to calculate the metadata account address for a mint
pub fn get_metadata_address(mint_pubkey: &Pubkey) -> Pubkey {
    let metadata_program = Pubkey::from_str(METADATA_PROGRAM_ID).unwrap();
    let seeds = &[
        b"metadata".as_ref(),
        metadata_program.as_ref(),
        mint_pubkey.as_ref(),
    ];

    let (metadata_pubkey, _) = Pubkey::find_program_address(seeds, &metadata_program);
    metadata_pubkey
}

// Retrieve token metadata (name, symbol) from the chain
pub fn get_token_metadata(client: &RpcClient, mint_pubkey: &Pubkey) -> Option<TokenMetadata> {
    let metadata_pubkey = get_metadata_address(mint_pubkey);
    if let Ok(account_metadata) = client.get_account_data(&metadata_pubkey) {
        if let Ok(metadata) = TokenMetadata::try_from_slice(&account_metadata) {
            return Some(metadata);
        }
    }
    None
}

// Function to extract mint public key from a transaction
pub fn get_mint_pubkey(client: &RpcClient, signature: &Signature) -> Option<Pubkey> {
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: None,
        max_supported_transaction_version: None,
    };

    if let Ok(transaction) = client.get_transaction_with_config(signature, config) {
        return extract_mint_pubkey_from_transaction(&transaction);
    }
    None
}

fn extract_mint_pubkey_from_transaction(
    transaction: &EncodedConfirmedTransactionWithStatusMeta,
) -> Option<Pubkey> {
    if let Some(meta) = &transaction.transaction.meta {
        if let OptionSerializer::Some(inner_instructions) = &meta.inner_instructions {
            for inner_instruction_set in inner_instructions {
                if let Some(mint_pubkey) =
                    process_inner_instructions(&inner_instruction_set.instructions)
                {
                    return Some(mint_pubkey);
                }
            }
        }
    }
    None
}

fn process_inner_instructions(instructions: &[UiInstruction]) -> Option<Pubkey> {
    for instruction in instructions {
        if let UiInstruction::Parsed(UiParsedInstruction::Parsed(parsed_instruction)) = instruction
        {
            if parsed_instruction.program == "spl-token"
                && parsed_instruction.parsed["type"].as_str() == Some("initializeMint")
            {
                if let Some(mint) = parsed_instruction.parsed["info"]["mint"].as_str() {
                    return Pubkey::from_str(mint).ok();
                }
            }
        }
    }
    None
}
