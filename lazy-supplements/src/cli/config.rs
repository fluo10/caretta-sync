use std::path::PathBuf;

use clap::Args;
use libp2p::identity;

use crate::{config::{NodeConfig, RawNodeConfig}, error::Error, global::{DEFAULT_CONFIG_FILE_PATH, DEFAULT_RAW_NODE_CONFIG}};

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[command(flatten)]
    pub node_config: RawNodeConfig,
}

impl ConfigArgs {
    pub fn get_config_path_or_default(&self) -> PathBuf {
        if let Some(x) = self.config.as_ref() {
            x.clone()
        } else {
            DEFAULT_CONFIG_FILE_PATH.to_path_buf()
        }
    }
    pub async fn try_into_raw_node_config(self) -> Result<RawNodeConfig, Error> {
        Ok(RawNodeConfig::read_from(self.get_config_path_or_default()).await? + self.node_config)
    }
    pub async fn try_into_node_config(self) -> Result<NodeConfig, Error> {
        Ok((DEFAULT_RAW_NODE_CONFIG.clone() + self.try_into_raw_node_config().await?).try_into()?)
    }
}