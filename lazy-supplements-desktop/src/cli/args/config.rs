use std::{net::IpAddr, path::PathBuf};

use clap::Args;
use lazy_supplements_core::config::ConfigError;
use crate::config::{PartialP2pConfig, PartialStorageConfig};
#[cfg(unix)]
use crate::config::PartialUnixConfig;

use serde::{Deserialize, Serialize};

use crate::{
    config::PartialDesktopConfig,
    error::Error,
    global::{DEFAULT_CONFIG_FILE_PATH, DEFAULT_PARTIAL_CORE_CONFIG,}
};

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(short = "c", long = "config")]
    pub file_path: Option<PathBuf>,
    #[arg(skip)]
    pub file_content: Option<Result<PartialDesktopConfig, ConfigError>>,
    #[command(flatten)]
    pub args: PartialDesktopConfig,
}


impl ConfigArgs {
    pub fn get_file_path_or_default(&self) -> PathBuf {
        self.file_path.unwrap_or(DEFAULT_CONFIG_FILE_PATH)
    }
    pub async fn get_or_read_file_content(&mut self) -> Result<PartialDesktopConfig, ConfigError> {
        self.file_content.get_or_insert(
            PartialDesktopConfig::read_from(self.get_config_path_or_default()).await
        ).clone()
    }
    pub fn get_config_path_or_default(&self) -> PathBuf {
        if let Some(x) = self.config.as_ref() {
            x.clone()
        } else {
            DEFAULT_CONFIG_FILE_PATH.to_path_buf()
        }
    }
    pub async fn try_into_partial_core_config(self) -> Result<PartialCoreConfig, Error> {
        let mut config = PartialCoreConfig::read_from(self.get_config_path_or_default()).await?;
        config.merge(self.core_config.into());
        Ok(config)
    }
    pub async fn try_into_core_config(self) -> Result<CoreConfig, Error> {
        let mut config = DEFAULT_PARTIAL_CORE_CONFIG.clone();
        config.merge(self.try_into_partial_core_config().await?);
        config.try_into()
    }
}
