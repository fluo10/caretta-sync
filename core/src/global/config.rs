use tempfile::TempDir;
use tokio::sync::OnceCell;

use crate::{config::{Config, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, StorageConfig}, error::Error, global::GlobalConstant};

pub static CONFIG: GlobalConfig = GlobalConfig::const_new();
pub struct GlobalConfig {
    inner: OnceCell<Config>
}

impl GlobalConfig { 
    pub const fn const_new() -> Self {
        Self{
            inner: OnceCell::const_new()
        }
    }
    pub async fn get_or_init(&'static self, source: Config) -> &'static Config {
        self.inner.get_or_init(|| async {
            source
        }).await
    }
    pub fn get(&'static self) -> Option<&'static Config> {
        self.inner.get()
    }
    pub fn get_and_unwrap(&'static self) -> &'static Config {
        self.get().expect(&format!("Config is uninitialized!"))
    }
    #[cfg(any(test, feature=test))]
    pub async fn get_or_init_test(&'static self) -> &'static Config {
        let temp_dir = TempDir::new().unwrap().keep();
        let mut data_dir = temp_dir.clone();
        data_dir.push("data");
        let mut cache_dir = temp_dir;
        cache_dir.push("cache");


        self.get_or_init(Config {
            p2p: PartialP2pConfig::default().with_new_secret().try_into().unwrap(),
            storage: StorageConfig {
                data_directory: data_dir,
                cache_directory: cache_dir,
            },
            rpc: PartialRpcConfig::default().try_into().unwrap(),
        }).await
    }
}