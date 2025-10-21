pub mod error;
mod iroh;
mod rpc;
mod storage;

use crate::utils::{emptiable::Emptiable, mergeable::Mergeable};
pub use error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub use iroh::{IrohConfig, PartialIrohConfig};
pub use rpc::*;
pub use storage::{PartialStorageConfig, StorageConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "cli")]
use clap::Args;

#[derive(Clone, Debug)]
pub struct Config {
    pub iroh: IrohConfig,
    pub storage: StorageConfig,
    pub rpc: RpcConfig,
}

impl AsRef<StorageConfig> for Config {
    fn as_ref(&self) -> &StorageConfig {
        &self.storage
    }
}

impl AsRef<IrohConfig> for Config {
    fn as_ref(&self) -> &IrohConfig {
        &self.iroh
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
        Ok(Self {
            rpc: value
                .rpc
                .ok_or(crate::error::Error::MissingConfig("rpc"))?
                .try_into()?,
            iroh: value
                .iroh
                .ok_or(crate::error::Error::MissingConfig("p2p"))?
                .try_into()?,
            storage: value
                .storage
                .ok_or(crate::error::Error::MissingConfig("storage"))?
                .try_into()?,
        })
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialConfig {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub iroh: Option<PartialIrohConfig>,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub storage: Option<PartialStorageConfig>,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub rpc: Option<PartialRpcConfig>,
}

impl Default for PartialConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialConfig {
    pub fn new() -> Self {
        Self {
            iroh: Some(PartialIrohConfig::empty().with_new_secret_key()),
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
    pub fn read_or_create<T>(path: T) -> Result<Self, ConfigError>
    where
        T: AsRef<Path>,
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path)?;
        }
        Self::read_from(&path)
    }
    pub fn read_from<T>(path: T) -> Result<Self, ConfigError>
    where
        T: AsRef<Path>,
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path)?;
        }
        let mut file = File::open(path.as_ref())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn write_to<T>(&self, path: T) -> Result<(), ConfigError>
    where
        T: AsRef<Path>,
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path)?;
        }
        let mut file = File::create(&path)?;
        file.write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }
    pub fn default(app_name: &'static str) -> Self {
        Self {
            iroh: Some(PartialIrohConfig::default()),
            rpc: Some(PartialRpcConfig::default(app_name)),
            storage: Some(PartialStorageConfig::default(app_name)),
        }
    }
}

impl From<Config> for PartialConfig {
    fn from(value: Config) -> Self {
        Self {
            iroh: Some(value.iroh.into()),
            storage: Some(value.storage.into()),
            rpc: Some(value.rpc.into()),
        }
    }
}

impl Emptiable for PartialConfig {
    fn empty() -> Self {
        Self {
            iroh: None,
            storage: None,
            rpc: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.iroh.is_empty() && self.rpc.is_empty() && self.storage.is_empty()
    }
}

impl Mergeable for PartialConfig {
    fn merge(&mut self, other: Self) {
        self.iroh.merge(other.iroh);
        self.rpc.merge(other.rpc);
        self.storage.merge(other.storage);
    }
}
