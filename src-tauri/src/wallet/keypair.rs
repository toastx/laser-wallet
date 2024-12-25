#![allow(unused)]

use serde::ser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::keypair_from_seed;
use solana_sdk::signer::keypair;
use solana_sdk::signer::Signer;
use aes::cipher::
{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use tauri::State;

use crate::AppState; 


#[derive(Serialize, Deserialize)]
pub struct NewWallet {
    seed: Vec<u8>,
    public_key: Pubkey,
}

pub fn encrypt_wallet(password: String, seed: Vec<u8>) -> Result<Vec<u8>, String> {
    let key = password.as_bytes();
    let aes_key = GenericArray::from_slice(key);
    let serialized = seed;
    println!("serialized: {:?}", serialized);
    println!("{}",serialized.len());

    if serialized.len() != 64 {
        return Err("Serialized wallet data must be 64 bytes.".into());
    }
    if key.len() < 32 {
        return Err("Password must be 8 bytes long.".into());
    }
    let cipher = Aes256::new(&aes_key);
    let mut encrypted = Vec::new();
    for chunk in serialized.chunks(16) { // Safely split into 16-byte chunks
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    Ok(encrypted)
}


pub fn decrypt_wallet(password: String, data: Vec<u8>) -> Result<Vec<u8>, String> {
    
    if password.len() != 32 {
        return Err("Password must be exactly 32 bytes.".into());
    }
    if data.len() != 64 {
        return Err("Encrypted data must be 64 bytes.".into());
    }
    let key = password.as_bytes();
    let aes_key = GenericArray::from_slice(key);
    let cipher = Aes256::new(&aes_key);
    let mut decrypted = Vec::new();
    for chunk in data.chunks(16) { // Safely split into 16-byte chunks
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }
    Ok(decrypted.to_vec())
}


#[tauri::command]
pub fn generate_wallet(state: State<AppState>) -> NewWallet {
    let keypair = keypair::Keypair::new();
    let seed = keypair::Keypair::to_bytes(&keypair);
    let trimmed_seed = &seed[0..32];
    println!("seed: {:?}", seed);
    let mut seed_lock = state.seed.lock().unwrap();
    *seed_lock = Some(seed.to_vec());
    println!("seed_lock: {:?}", seed_lock);
    let pubkey = keypair.pubkey();
    

    NewWallet {
        seed: trimmed_seed.to_vec(),
        public_key: pubkey,
    }
}

#[tauri::command]
pub fn get_wallet(state: State<AppState>) -> Result<Pubkey, String> {
    let seed_lock = state.seed.lock().unwrap();
    let seed = seed_lock.clone().unwrap();
    let decrypted_seed = decrypt_wallet("passwordpasswordpasswordpassword".to_string(), seed).unwrap();
    println!("decrypted_seed: {:?}", decrypted_seed);
    let wallet = keypair::keypair_from_seed(&decrypted_seed).unwrap();
    let pubkey = wallet.pubkey();
    Ok(pubkey)
}

