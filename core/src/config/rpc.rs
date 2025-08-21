use std::{net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}, path::PathBuf, str::FromStr};
#[cfg(feature="desktop")]
use clap::Args;
use crate::{config::PartialConfig, utils::{emptiable::Emptiable, mergeable::Mergeable}};
use libp2p::mdns::Config;
use serde::{Deserialize, Serialize};

use crate::config::error::ConfigError;

#[cfg(unix)]
static DEFAULT_SOCKET_PATH: &str = "caretta.sock";

#[derive(Clone, Debug)]
pub struct RpcConfig {
    pub socket_path: PathBuf,
}

impl TryFrom<PartialRpcConfig> for RpcConfig {
    type Error = ConfigError;
    fn try_from(config: PartialRpcConfig) -> Result<Self, Self::Error> {
        Ok(Self{
            socket_path: config.socket_path.ok_or(ConfigError::MissingConfig("port".to_string()))?,
        })
    }
}

#[cfg_attr(feature="desktop", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialRpcConfig {
    pub socket_path: Option<PathBuf>,
}

impl Default for PartialRpcConfig {
    fn default() -> Self {
        Self{
            socket_path: Some(PathBuf::from_str(DEFAULT_SOCKET_PATH).unwrap()),
        }
    }
}

impl Emptiable for PartialRpcConfig {
    fn empty() -> Self {
        Self {
            socket_path: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.socket_path.is_none()
    }
}

impl From<RpcConfig> for PartialRpcConfig {
    fn from(source: RpcConfig) -> Self {
        Self {
            socket_path: Some(source.socket_path),
        }
    }
}

impl Mergeable for PartialRpcConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.socket_path {
            self.socket_path = Some(x);
        }
    }
}

impl Mergeable for Option<PartialRpcConfig> {
    fn merge(&mut self, mut other: Self) {
        match other.take() {
            Some(x) => {
                if let Some(y) = self.as_mut() {
                    y.merge(x);
                } else {
                    let _ = self.insert(x);
                }
            },
            None => {}
        };
    }
}