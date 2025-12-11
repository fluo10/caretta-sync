use std::{net::{IpAddr, SocketAddr}, path::PathBuf};

use chrono::format::Parsed;
use clap::Args;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::parsed_config::error::ParsedConfigError;
use caretta_sync_core::{
    config::{IpcConfig,},
    util::{Emptiable, Mergeable},
};

impl TryFrom<ParsedIpcConfig> for IpcConfig {
    type Error = ParsedConfigError;
    fn try_from(config: ParsedIpcConfig) -> Result<Self, Self::Error> {
        if let Some(x) = config.endpoint_addr {
            Ok(Self{
                endpoint: x
            })
        } else {
            Err(ParsedConfigError::MissingConfig("ip_socket_addr (or unix_socket_path)"))
        }
    }
}

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ParsedIpcConfig {
    #[arg(long = "ipc-endpoint-addr", env = "IPC_ENDPOINT_ADDR")]
    pub endpoint_addr: Option<SocketAddr>,
}

impl ParsedIpcConfig {
    pub fn default(app_name: &'static str) -> Self {
        IpcConfig::default(app_name).into()
    }
}

impl Emptiable for ParsedIpcConfig {
    fn empty() -> Self {
        Self { 
            endpoint_addr: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.endpoint_addr.is_none()
    }
}

impl From<IpcConfig> for ParsedIpcConfig {
    fn from(source: IpcConfig) -> Self {
        Self {
            endpoint_addr: Some(source.endpoint),
        }
    }
}

impl Mergeable for ParsedIpcConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.endpoint_addr {
            self.endpoint_addr = Some(x);
        }

    }
}