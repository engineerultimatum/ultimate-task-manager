use crate::models::SaveData;

/// High-level save management API
pub struct SaveManager;

impl SaveManager {
    /// Load save data for a given seed
    pub fn load_save(seed: &str) -> Option<SaveData> {
        #[cfg(target_arch = "wasm32")]
        {
            super::web_storage::load_save(seed)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            super::file_storage::load_save(seed)
        }
    }

    /// Save data to storage
    pub fn save_data(data: &SaveData) {
        #[cfg(target_arch = "wasm32")]
        {
            super::web_storage::save_data_to_storage(data);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            super::file_storage::save_data_to_storage(data);
        }
    }
}
