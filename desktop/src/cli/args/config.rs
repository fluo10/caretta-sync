use std::{net::IpAddr, path::PathBuf};

use clap::Args;
use caretta_core::config::{PartialConfig,PartialP2pConfig, PartialStorageConfig, ConfigError};

use serde::{Deserialize, Serialize};

use crate::global::DEFAULT_CONFIG_FILE_PATH;

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(short = 'c', long = "config")]
    pub file_path: Option<PathBuf>,
    #[arg(skip)]
    pub file_content: Option<PartialConfig>,
    #[command(flatten)]
    pub args: PartialConfig,
}


impl ConfigArgs {
    pub fn get_file_path_or_default(&self) -> PathBuf {
        self.file_path.clone().unwrap_or((*DEFAULT_CONFIG_FILE_PATH).clone())
    }
    pub async fn get_or_read_file_content(&mut self) -> &mut PartialConfig {
        self.file_content.get_or_insert(
            PartialConfig::read_from(self.get_file_path_or_default()).await.unwrap()
        )
    }
}
