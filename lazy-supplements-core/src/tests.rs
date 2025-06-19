use std::{path::PathBuf, sync::LazyLock};

use tempfile::TempDir;
use crate::{ config::PartialConfig, message::Message};

use serde::{de::DeserializeOwned, Deserialize, Serialize};


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

pub fn test_cbor_serialize_deserialize<T>(src: T)
where T: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug
{
    let mut buf: Vec<u8> = Vec::new();
    ciborium::into_writer(&src, &mut buf).unwrap();
    let dst: T = ciborium::from_reader(buf.as_slice()).unwrap();
    assert_eq!(src, dst);
}

pub fn test_toml_serialize_deserialize<T>(src: T)
where T: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug
{
    let buf = toml::to_string(&src).unwrap();
    let dst: T = toml::from_str(&buf).unwrap();
    assert_eq!(src, dst);
}