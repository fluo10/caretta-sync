use std::{net::IpAddr, path::PathBuf, sync::LazyLock};

use clap::Args;
use caretta_core::{
    config::{Config, ConfigError, PartialConfig, PartialP2pConfig, PartialStorageConfig},
    utils::mergeable::Mergeable
};

use serde::{Deserialize, Serialize};

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
    pub fn get_file_path_or_default(&self, app_name: &'static str) -> PathBuf {
        self.file_path.clone().unwrap_or(
            dirs::config_local_dir()
                .unwrap()
                .join(app_name)
                .join(app_name.to_string() + ".conf")
        )
    }
    pub async fn get_or_read_file_content(&mut self, app_name: &'static str) -> PartialConfig {
        self.file_content.get_or_insert(
            PartialConfig::read_from(self.get_file_path_or_default(app_name)).await.unwrap()
        ).clone()
    }
    pub async fn into_config_unchecked(mut self, app_name: &'static str) -> Config {
        let mut default = PartialConfig::default_desktop(app_name);
        let file_content = self.get_or_read_file_content(app_name).await;
        let args = self.args;
        default.merge(file_content);
        default.merge(args);
        default.try_into().unwrap()

    } 
}
