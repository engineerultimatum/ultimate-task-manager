use crate::models::SaveData;
use std::path::PathBuf;

/// Load save data from desktop filesystem
pub fn load_save(seed: &str) -> Option<SaveData> {
    use std::fs;
    
    let save_path = get_save_path(seed)?;
    
    if !save_path.exists() {
        return None;
    }
    
    match fs::read_to_string(&save_path) {
        Ok(content) => serde_json::from_str(&content).ok(),
        Err(_) => None,
    }
}

/// Save data to desktop filesystem
pub fn save_data_to_storage(data: &SaveData) {
    use std::fs;
    
    if let Some(save_path) = get_save_path(&data.seed) {
        // Create directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Write save file
        if let Ok(json) = serde_json::to_string_pretty(data) {
            let _ = fs::write(&save_path, json);
        }
    }
}

/// Get the save file path for a given seed
fn get_save_path(seed: &str) -> Option<PathBuf> {
    let config_dir = dirs::config_dir()?;
    let save_file = format!("{}.json", seed.replace("/", "_").replace("\\", "_"));
    
    Some(config_dir.join("hot_dog").join("saves").join(save_file))
}
