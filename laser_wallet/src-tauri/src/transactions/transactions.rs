use std::str::FromStr;

use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::signer::keypair::Keypair;
use solana_client::rpc_client::RpcClient;
use solana_sdk::system_instruction;
use solana_sdk::hash::Hash;
use tauri::State;
use crate::AppState;


pub fn create_connection() -> Result<RpcClient, String> {
    let url = "https://api.devnet.solana.com".to_string();
    Ok(RpcClient::new(url))
}

pub fn generate_keypair(state: &State<AppState>) -> Result<Keypair, String> {
    let seed_lock = state.seed.lock().unwrap();
    let seed = seed_lock.clone().unwrap();
    let trimmed_seed = &seed[0..32];
    if trimmed_seed.len() != 32 {
        return Err("Seed must be 32 bytes long.".to_string());
    }
    Ok(Keypair::from_seed(&trimmed_seed).map_err(|_| "Failed to generate keypair from seed.")?)
}

pub fn transfer_asset(
    state: &State<AppState>,
    connection: &RpcClient,
    recipient_pubkey: String,
    lamports: u64,
) -> Result<Transaction, String> {
    
    let rec = Pubkey::from_str(&recipient_pubkey).map_err(|err| format!("Failed to parse recipient pubkey: {:?}", err))?;
    let sender_keypair = generate_keypair(state)?;
    let recent_blockhash = connection
        .get_latest_blockhash()
        .map_err(|err| format!("Failed to get recent blockhash: {:?}", err))?;

    let blockhash = Hash::new_from_array(recent_blockhash.to_bytes());    
    
    let transfer_instruction = system_instruction::transfer(
        &sender_keypair.pubkey(),
        &rec,
        lamports,
    );

    let mut tx = Transaction::new_unsigned(
        Message::new(
            &[transfer_instruction],
            Some(&sender_keypair.pubkey())
        )
    );
    tx.sign(&[&sender_keypair], blockhash);
    
    Ok(tx)
}


