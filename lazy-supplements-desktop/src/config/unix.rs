use std::path::PathBuf;
use clap::Args;
use lazy_supplements_core::{config::PartialConfig, utils::{emptiable::Emptiable, mergeable::Mergeable}};
use libp2p::mdns::Config;
use serde::{Deserialize, Serialize};

use crate::config::error::ConfigError;


pub struct UnixConfig {
    pub socket_path: PathBuf,
}

impl TryFrom<PartialUnixConfig> for UnixConfig {
    type Error = ConfigError;
    fn try_from(config: PartialUnixConfig) -> Result<Self, Self::Error> {
        Ok(Self{
            socket_path: config.socket_path.ok_or(ConfigError::MissingConfig("socket_path".to_string()))?
        })
    }
}

#[derive(Args, Clone, Debug, Deserialize, Emptiable, Mergeable, Serialize)]
pub struct PartialUnixConfig {
    pub socket_path: Option<PathBuf>,
}

impl Default for PartialUnixConfig {
    fn default() -> Self {
        todo!()
    }
}

impl From<UnixConfig> for PartialUnixConfig {
    fn from(source: UnixConfig) -> Self {
        Self {
            socket_path: Some(source.socket_path)
        }
    }
}


