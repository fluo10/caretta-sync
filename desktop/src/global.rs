use std::{path::PathBuf, sync::LazyLock};

pub use caretta_core::global::*;

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
