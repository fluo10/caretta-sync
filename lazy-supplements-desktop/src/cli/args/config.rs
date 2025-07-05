use std::{net::IpAddr, path::PathBuf};

use clap::Args;
use lazy_supplements_core::config::{BaseConfig, ConfigError};
use crate::config::{PartialP2pConfig, PartialStorageConfig};
#[cfg(unix)]
use crate::config::PartialUnixConfig;

use serde::{Deserialize, Serialize};

use crate::{
    config::DesktopBaseConfig,
    error::Error,
    global::DEFAULT_CONFIG_FILE_PATH
};

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(short = 'c', long = "config")]
    pub file_path: Option<PathBuf>,
    #[arg(skip)]
    pub file_content: Option<DesktopBaseConfig>,
    #[command(flatten)]
    pub args: DesktopBaseConfig,
}


impl ConfigArgs {
    pub fn get_file_path_or_default(&self) -> PathBuf {
        self.file_path.clone().unwrap_or((*DEFAULT_CONFIG_FILE_PATH).clone())
    }
    pub async fn get_or_read_file_content(&mut self) -> &mut DesktopBaseConfig {
        self.file_content.get_or_insert(
            DesktopBaseConfig::read_from(self.get_file_path_or_default()).await.unwrap()
        )
    }
}
