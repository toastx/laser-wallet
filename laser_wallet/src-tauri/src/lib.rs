mod wallet;
use tauri::State;

mod fs;
use tauri_plugin_fs::FsExt;
use std::sync::Mutex;


#[derive(Default)]
struct AppState {
    seed: Mutex<Option<Vec<u8>>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn store_wallet(state: State<AppState>,password: String) -> Result<(), String> {
    let seed = get_seed(state.clone()).unwrap();
    let encrypted_seed = wallet::encrypt_wallet(password, seed).unwrap();
    fs::save_seed(encrypted_seed, &state)?;
    Ok(())
}

#[tauri::command]
fn fetch_wallet(state: State<AppState>,password: String) -> Result<(bool), String> {
    
    let encrypted_seed = fs::load_seed(&state).expect("Failed to load seed");
    let decrypted_seed = wallet::decrypt_wallet(password, encrypted_seed).expect("Failed to decrypt seed");
    let mut seed_lock = state.seed.lock().expect("Failed to lock seed");
    *seed_lock = Some(decrypted_seed.clone());  
    Ok(true)
}

#[tauri::command]
fn get_seed(state: State<AppState>) -> Result<Vec<u8>, String> {
    let seed_lock = state.seed.lock().unwrap();
    Ok(seed_lock.clone().unwrap())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            seed: Mutex::new(Some(vec![0,64])),
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, wallet::generate_wallet,wallet::get_wallet,store_wallet,fetch_wallet,get_seed])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory("../user_data/", false);
            dbg!(scope.allowed());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



