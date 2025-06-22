use std::path::PathBuf;

#[cfg(feature="desktop")]
use clap::Args;
use crate::{config::{ConfigError, PartialConfig}};
use libp2p::mdns::Config;
use serde::{Deserialize, Serialize};

static DATA_DATABASE_NAME: &str = "data.sqlite";
static CACHE_DATABASE_NAME: &str = "cache.sqlite";

#[cfg(any(test, feature="test"))]
use crate::tests::{GlobalTestDefault, TestDefault};

#[derive(Debug)]
pub struct StorageConfig {
    pub data_directory: PathBuf,
    pub cache_directory: PathBuf,
}

impl StorageConfig {
    pub fn get_data_database_path(&self) -> PathBuf{
        self.data_directory.join(DATA_DATABASE_NAME)
    }
    pub fn get_cache_database_path(&self) -> PathBuf {
        self.cache_directory.join(CACHE_DATABASE_NAME)
    }
}

#[cfg(any(test, feature="test"))]
impl TestDefault for StorageConfig {
    fn test_default() -> Self {
        let temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path().keep().unwrap();
        Self { data_directory: temp_path.clone(), cache_directory: temp_path }
    }
}

impl TryFrom<PartialStorageConfig> for StorageConfig {
    type Error = ConfigError;

    fn try_from(value: PartialStorageConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            data_directory: value.data_directory.ok_or(ConfigError::MissingConfig("data_directory".to_string()))?,
            cache_directory: value.cache_directory.ok_or(ConfigError::MissingConfig("cache_directory".to_string()))?,
        })
    }
}
#[cfg_attr(feature="desktop", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialStorageConfig {
    #[cfg_attr(feature="desktop", arg(long))]
    pub data_directory: Option<PathBuf>,
    #[cfg_attr(feature="desktop", arg(long))]
    pub cache_directory: Option<PathBuf>,
}

impl From<StorageConfig> for PartialStorageConfig {
    fn from(config: StorageConfig) -> PartialStorageConfig {
        Self {
            data_directory: Some(config.data_directory),
            cache_directory: Some(config.cache_directory),
        }
    }
}

impl PartialConfig for PartialStorageConfig {
    fn empty() -> Self {
        Self{
            data_directory: None,
            cache_directory: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.data_directory.is_none() && self.cache_directory.is_none()
    }
    fn default() -> Self {
        todo!()
    }
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.data_directory {
            self.data_directory = Some(x);
        }
        if let Some(x) = other.cache_directory {
            self.cache_directory = Some(x);
        }
    }
}