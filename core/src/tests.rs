use std::{path::PathBuf, sync::LazyLock};

use tempfile::TempDir;
use url::Url;
use crate::{ config::{Config, PartialConfig, PartialIrohConfig, PartialRpcConfig, RpcConfig, StorageConfig}};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub static TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let test_dir = TempDir::new().unwrap().keep();
    let data_dir = test_dir.join("data");
    let cache_dir = test_dir.join("cache");


    Config {
        iroh: PartialIrohConfig::default().with_new_secret_key().try_into().unwrap(),
        storage: StorageConfig {
            data_directory: data_dir,
            cache_directory: cache_dir,
        },
        rpc: RpcConfig{
            endpoint_url: Url::parse(&(String::from("unix://") + test_dir.join("socket.sock").to_str().unwrap())).unwrap(),
        },
    }
});
