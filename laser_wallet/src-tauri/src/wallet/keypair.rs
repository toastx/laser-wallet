#![allow(unused)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair;
use solana_sdk::signer::Signer;
use aes::cipher::
{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;


#[derive(Serialize, Deserialize)]
pub struct NewWallet {
    seed: Vec<u8>,
    public_key: Pubkey,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub seed:Vec<u8>
}

pub fn encrypt_wallet(password: String, wallet: Wallet) -> Result<Vec<u8>, String> {
    let key = password.as_bytes();
    let aes_key = GenericArray::from_slice(key);
    let serialized = serde_json::to_vec(&wallet).unwrap();
    let mut data = GenericArray::clone_from_slice(&serialized);
    if key.len() <= 32 {
        return Err("Password must be 32 bytes long.".into());
    }
    let cipher = Aes256::new(&aes_key);
    cipher.encrypt_block(&mut data);
    Ok(data.to_vec())
}


pub fn decrypt_wallet(password: String, data: Vec<u8>) -> Result<Vec<u8>, String> {
    let key = password.as_bytes();
    let aes_key = GenericArray::from_slice(key);
    let mut block_cipher = GenericArray::clone_from_slice(&data);
    let cipher = Aes256::new(&aes_key);
    cipher.decrypt_block(&mut block_cipher);
    Ok(block_cipher.to_vec())
}


#[tauri::command]
pub fn generate_wallet() -> NewWallet {
    let keypair = keypair::Keypair::new();
    let seed = keypair::Keypair::to_bytes(&keypair);
    let trimmed_seed = &seed[0..32];
    println!("seed: {:?}", seed);
    let pubkey = keypair.pubkey();
    let wallet = Wallet { seed: trimmed_seed.to_vec() };

    NewWallet {
        seed: trimmed_seed.to_vec(),
        public_key: pubkey,
    }
}
