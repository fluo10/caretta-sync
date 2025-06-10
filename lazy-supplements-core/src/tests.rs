use std::{path::PathBuf, sync::LazyLock};

use tempfile::TempDir;

pub static TEST_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let pkg_name = env!("CARGO_PKG_NAME");
    let timestamp = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, false);
    std::env::temp_dir().join(pkg_name).join( &timestamp)
});

pub static TEST_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    TempDir::new().unwrap().keep()
});

pub static TEST_DATABASE_PATH: std::sync::LazyLock<PathBuf> = std::sync::LazyLock::new(|| {
    TEST_DIR_PATH.join("lazy-supplements.sqlite")
});
