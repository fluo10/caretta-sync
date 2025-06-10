use std::{net::IpAddr, path::PathBuf};

use clap::Args;
use lazy_supplements_core::config::RawNodeConfig;
use serde::{Deserialize, Serialize};

use crate::{config::NodeConfig, error::Error, global::{DEFAULT_CONFIG_FILE_PATH, DEFAULT_RAW_NODE_CONFIG}};

#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[command(flatten)]
    pub config_values: ConfigValueArgs,
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
        Ok(RawNodeConfig::read_from(self.get_config_path_or_default()).await? + self.config_values.into())
    }
    pub async fn try_into_node_config(self) -> Result<NodeConfig, Error> {
        Ok((DEFAULT_RAW_NODE_CONFIG.clone() + self.try_into_raw_node_config().await?).try_into()?)
    }
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct ConfigValueArgs {
    #[arg(skip)]
    pub secret: Option<String>,
    #[arg(long)]
    pub database_path: Option<PathBuf>,
    #[arg(long)]
    pub listen_ips: Option<Vec<IpAddr>>,
    #[arg(long)]
    pub port: Option<u16>,
}

impl Into<RawNodeConfig> for ConfigValueArgs {
    fn into(self) -> RawNodeConfig {
        RawNodeConfig {
            secret : self.secret,
            database_path: self.database_path,
            listen_ips: self.listen_ips,
            port: self.port
        }
    }
}