use std::{net::IpAddr, path::PathBuf};

use clap::Args;
use lazy_supplements_core::config::{PartialConfig, PartialCoreConfig};
use serde::{Deserialize, Serialize};

use crate::{config::{desktop::PartialDesktopConfig, CoreConfig}, error::Error, global::{DEFAULT_CONFIG_FILE_PATH, DEFAULT_PARTIAL_CORE_CONFIG,}};

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[command(flatten)]
    pub core_config: PartialCoreConfig,
    #[command(flatten)]
    pub desktop_config: PartialDesktopConfig,
}


impl ConfigArgs {
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
