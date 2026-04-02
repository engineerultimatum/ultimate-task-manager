pub mod save_manager;

#[cfg(target_arch = "wasm32")]
pub mod web_storage;

#[cfg(not(target_arch = "wasm32"))]
pub mod file_storage;

pub use save_manager::*;
