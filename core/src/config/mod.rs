pub mod error;
mod storage;
mod p2p;
mod rpc;

use std::{path::Path, default::Default};
use crate::{utils::{emptiable::Emptiable, mergeable::Mergeable}};
pub use error::ConfigError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
pub use storage::{StorageConfig, PartialStorageConfig};
pub use p2p::{P2pConfig, PartialP2pConfig};
pub use rpc::*;

#[cfg(feature="cli")]
use clap::Args;

#[derive(Clone, Debug)]
pub struct Config {
    pub p2p: P2pConfig,
    pub storage: StorageConfig,
    pub rpc: RpcConfig,
}

impl AsRef<StorageConfig> for Config {
    fn as_ref(&self) -> &StorageConfig {
        &self.storage
    }
}

impl AsRef<P2pConfig> for Config {
    fn as_ref(&self) -> &P2pConfig {
        &self.p2p
    }
}

impl AsRef<RpcConfig> for Config {
    fn as_ref(&self) -> &RpcConfig {
        &self.rpc
    }
}

impl TryFrom<PartialConfig> for Config {
    type Error = crate::error::Error;
    fn try_from(value: PartialConfig) -> Result<Self, Self::Error> {
        Ok(Self{
            rpc: value.rpc.ok_or(crate::error::Error::MissingConfig("rpc"))?.try_into()?,
            p2p: value.p2p.ok_or(crate::error::Error::MissingConfig("p2p"))?.try_into()?,
            storage: value.storage.ok_or(crate::error::Error::MissingConfig("storage"))?.try_into()?
        })
    }
}

#[cfg_attr(feature="cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialConfig {
    #[cfg_attr(feature="cli", command(flatten))]
    pub p2p: Option<PartialP2pConfig>,
    #[cfg_attr(feature="cli", command(flatten))]
    pub storage: Option<PartialStorageConfig>,
    #[cfg_attr(feature="cli", command(flatten))]
    pub rpc: Option<PartialRpcConfig>,
}

impl PartialConfig {
    pub fn new() -> Self {
        Self {
            p2p : Some(PartialP2pConfig::empty().with_new_private_key()),
            storage: Some(PartialStorageConfig::empty()),
            rpc: Some(PartialRpcConfig::empty()),
        }
    }
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
    pub fn into_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }
    pub async fn read_or_create<T>(path: T) -> Result<Self, ConfigError> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path).await?;
        }
        Self::read_from(&path).await
    }
    pub async fn read_from<T>(path:T) -> Result<Self, ConfigError> 
    where 
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path).await?;
        }
        let mut file = File::open(path.as_ref()).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    pub async fn write_to<T>(&self, path:T) -> Result<(), ConfigError> 
    where 
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path).await?;
        }
        let mut file = File::create(&path).await?;
        file.write_all(toml::to_string(self)?.as_bytes()).await?;
        Ok(())
    }
    pub fn default(app_name: &'static str) -> Self {
        Self {
            p2p: Some(PartialP2pConfig::default()),
            rpc: Some(PartialRpcConfig::default(app_name)),
            storage: Some(PartialStorageConfig::default(app_name)),
        }
    }
}

impl From<Config> for PartialConfig {
    fn from(value: Config) -> Self {
        Self {
            p2p: Some(value.p2p.into()),
            storage: Some(value.storage.into()),
            rpc: Some(value.rpc.into())
        }
    }
}

impl Emptiable for PartialConfig {
    fn empty() -> Self {
        Self {
            p2p: None, 
            storage: None,
            rpc: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.rpc.is_empty() && self.storage.is_empty()
    }
}

impl Mergeable for PartialConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.rpc.merge(other.rpc);
        self.storage.merge(other.storage);
    }
}
