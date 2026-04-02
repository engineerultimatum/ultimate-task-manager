use crate::models::SaveData;

/// Web platform storage using localStorage
pub fn load_save(seed: &str) -> Option<SaveData> {
    use web_sys::window;
    
    let storage = window()?.local_storage().ok()??;
    let key = format!("save_{}", seed);
    
    match storage.get_item(&key).ok()? {
        Some(data) => serde_json::from_str(&data).ok(),
        None => None,
    }
}

/// Save data to localStorage
pub fn save_data_to_storage(data: &SaveData) {
    use web_sys::window;
    
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let key = format!("save_{}", data.seed);
            if let Ok(json) = serde_json::to_string(data) {
                let _ = storage.set_item(&key, &json);
            }
        }
    }
}
