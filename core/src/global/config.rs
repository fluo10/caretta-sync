#[cfg(any(test,feature="test"))]
use tempfile::TempDir;
use tokio::sync::OnceCell;

use crate::{config::{Config, ConfigError, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, StorageConfig}, error::Error};

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
    pub async fn get_or_init<T>(&'static self, config: T) -> &'static Config where 
    T: AsRef<Config>{
        self.inner.get_or_init(|| async {
            config.as_ref().clone()
        }).await
    }
    pub async fn get_or_try_init<T, E>(&'static self, config: T) -> Result<&'static Config, <T as TryInto<Config>>::Error> where 
        T: TryInto<Config>,
    {
        self.inner.get_or_try_init(|| async {
            config.try_into()
        }).await

    }
    pub fn get(&'static self) -> Option<&'static Config> {
        self.inner.get()
    }
}