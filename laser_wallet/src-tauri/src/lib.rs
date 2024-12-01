mod wallet;
use tauri::State;
use wallet::*;

mod fs;
use tauri_plugin_fs::FsExt;
use std::sync::Mutex;


#[derive(Default)]
struct AppState {
    seed: Mutex<Option<Vec<u8>>>,  // Use Mutex to safely mutate the seed across threads
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn store_wallet(password: String, seed: Vec<u8>) -> Result<(), String> {
    let encrypted_seed = wallet::encrypt_wallet(password, Wallet{seed: seed}).unwrap();
    fs::save_seed(encrypted_seed, state);
    Ok(())
}

#[tauri::command]
fn fetch_wallet(password: String) -> Result<(), String> {
    let encrypted_seed = fs::load_seed(state).unwrap();
    let decrypted_seed = wallet::decrypt_wallet(password, encrypted_seed).unwrap();
    let mut seed_lock = state.seed.lock().unwrap();
    *seed_lock = Some(decrypted_seed.clone());  
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, wallet::generate_wallet])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory("user_data", false);
            dbg!(scope.allowed());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



