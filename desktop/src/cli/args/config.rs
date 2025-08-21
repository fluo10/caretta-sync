use std::{net::IpAddr, path::PathBuf, sync::LazyLock};

use clap::Args;
use caretta_core::{
    config::{Config, ConfigError, PartialConfig, PartialP2pConfig, PartialStorageConfig},
    utils::{emptiable::Emptiable, mergeable::Mergeable}
};

use libp2p::identity::Keypair;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(short = 'c', long = "config")]
    pub file_path: Option<PathBuf>,
    #[arg(skip)]
    pub file_content: OnceCell<PartialConfig>,
    #[command(flatten)]
    pub args: PartialConfig,
}


impl ConfigArgs {
    fn get_file_path_or_default(&self, app_name: &'static str) -> PathBuf {
        self.file_path.clone().unwrap_or(
            dirs::config_local_dir()
                .expect("Config user directory should be set")
                .join(app_name)
                .join("config.toml")
        )
    }
    async fn get_or_read_file_content(&self, app_name: &'static str) -> PartialConfig {
        self.file_content.get_or_init(|| async {
            PartialConfig::read_from(self.get_file_path_or_default(app_name)).await.expect("Config file should be invalid!")
        }).await.clone()
    }
    pub async fn to_partial_config_with_default(&self, app_name: &'static str) -> PartialConfig {
        let mut default = PartialConfig::default_desktop(app_name);
        default.merge(self.to_partial_config_without_default(app_name).await);
        default
    }
    pub async fn to_partial_config_without_default(&self, app_name: &'static str) -> PartialConfig {
        let mut file_content = self.get_or_read_file_content(app_name).await;
        let args = self.args.clone();
        file_content.merge(args);
        file_content
    }
    async fn has_p2p_private_key(&self, app_name: &'static str) -> bool {
        let merged = self.to_partial_config_with_default(app_name).await;
        match merged.p2p {
            Some(p2p) => p2p.private_key.is_some(),
            None => false
        }
    }
    pub async fn into_config(mut self, app_name: &'static str) -> Config {
        if !self.has_p2p_private_key(app_name).await {
            let path = self.get_file_path_or_default(app_name);
            let mut content = self.file_content.get_mut().unwrap();
            if let Some(p2p) = content.p2p.as_mut() {
                p2p.init_private_key();
            } else {
                content.p2p.insert(PartialP2pConfig::empty().with_new_private_key());
            }
            content.write_to(path).await.expect("Config file should be writable first time to initialize secret");
        }
        self.to_partial_config_with_default(app_name).await.try_into().expect("Some configurations are missing!")
    } 
}