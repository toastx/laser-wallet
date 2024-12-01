use std::fs::{write, read};
use tauri::State;

use crate::AppState; 

pub fn save_seed(seed: Vec<u8>, state: State<AppState>) -> Result<(), String> {
    
    let mut seed_lock = state.seed.lock().unwrap();
    *seed_lock = Some(seed.clone());

    let path = "user_data/seed.txt";  
    match write(path, seed) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save seed: {}", e)),
    }
}

pub fn load_seed(state: &State<AppState>) -> Result<Vec<u8>, String> {
    
    let seed = state.seed.lock().unwrap();
    if let Some(ref s) = *seed {
        Ok(s.clone()) 
    } else {
        
        let path = "user_data/seed.txt";  
        match read(path) {
            Ok(seed) => {
                
                let mut seed_lock = state.seed.lock().unwrap();
                *seed_lock = Some(seed.clone());  
                Ok(seed)
            }
            Err(_) => Err("Seed file not found".into()),  
        }
    }
}



