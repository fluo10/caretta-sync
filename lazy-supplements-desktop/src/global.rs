use std::{path::PathBuf, sync::LazyLock};

use lazy_supplements_core::config::PartialCoreConfig;
pub use lazy_supplements_core::global::*;

pub static DEFAULT_DATA_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::data_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});
pub static DEFAULT_CONFIG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::config_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});

pub static DEFAULT_CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_CONFIG_DIR_PATH.join(&*DEFAULT_CONFIG_FILE_NAME)
});
pub static DEFAULT_DATABASE_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_DATA_DIR_PATH.join(&*DEFAULT_DATABASE_FILE_NAME)
});

pub static DEFAULT_PARTIAL_CORE_CONFIG: LazyLock<PartialCoreConfig> = LazyLock::new(|| {
    PartialCoreConfig {
        secret: None,
        listen_ips: Some(DEFAULT_LISTEN_IPS.to_vec()),
        port: Some(0),
    }
});