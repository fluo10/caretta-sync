use std::path::PathBuf;

use clap::Args;
use lazy_supplements_core::config::{ConfigError, PartialConfig};
use libp2p::mdns::Config;
use serde::{Deserialize, Serialize};

pub struct DesktopConfig {
    pub data_directory: PathBuf,
    pub data_database: PathBuf,
    pub cache_directory: PathBuf,
    pub cache_database: PathBuf,
}

impl TryFrom<PartialDesktopConfig> for DesktopConfig {
    type Error = ConfigError;

    fn try_from(value: PartialDesktopConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            data_directory: value.data_directory.ok_or(ConfigError::MissingConfig("data_directory".to_string()))?,
            data_database: value.data_database.ok_or(ConfigError::MissingConfig("data_database".to_string()))?,
            cache_directory: value.cache_directory.ok_or(ConfigError::MissingConfig("cache_directory".to_string()))?,
            cache_database: value.cache_database.ok_or(ConfigError::MissingConfig("cache_database".to_string()))?,
        })
    }
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct PartialDesktopConfig {
    #[arg(long)]
    pub data_directory: Option<PathBuf>,
    #[arg(long)]
    pub data_database: Option<PathBuf>,
    #[arg(long)]
    pub cache_directory: Option<PathBuf>,
    #[arg(long)]
    pub cache_database: Option<PathBuf>,
}

impl From<DesktopConfig> for PartialDesktopConfig {
    fn from(config: DesktopConfig) -> PartialDesktopConfig {
        Self {
            data_database: Some(config.data_database),
            data_directory: Some(config.data_directory),
            cache_database: Some(config.cache_database),
            cache_directory: Some(config.cache_directory),
        }
    }
}

impl PartialConfig<DesktopConfig> for PartialDesktopConfig {
    fn empty() -> Self {
        Self{
            data_database: None,
            cache_database: None,
            data_directory: None,
            cache_directory: None,
        }
    }
    fn default() -> Self {
        todo!()
    }
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.data_directory {
            self.data_directory = Some(x);
        }
        if let Some(x) = other.data_database {
            self.data_database = Some(x);
        }
        if let Some(x) = other.cache_directory {
            self.cache_directory = Some(x);
        }
        if let Some(x) = other.cache_database {
            self.cache_database = Some(x);
        }
    }
}