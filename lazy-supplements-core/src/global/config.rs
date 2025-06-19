use crate::{config::StorageConfig, error::Error};
use tokio::sync::OnceCell;

static STORAGE_CONFIG: OnceCell<StorageConfig> = OnceCell::const_new();

pub async fn get_or_init_storage_config(config: StorageConfig) -> &'static StorageConfig {
    STORAGE_CONFIG.get_or_init(|| async {
        config
    }).await
}

pub fn get_storage_config() -> Option<&'static StorageConfig> {
    STORAGE_CONFIG.get()
}

pub fn get_and_unwrap_storage_config() -> &'static StorageConfig {
    STORAGE_CONFIG.get().expect("global storage config is uninitialized!")
}
