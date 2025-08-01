use std::{net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}, path::PathBuf};
use clap::Args;
use lazy_supplements_core::{config::PartialConfig, utils::{emptiable::Emptiable, mergeable::Mergeable}};
use libp2p::mdns::Config;
use serde::{Deserialize, Serialize};

use crate::config::error::ConfigError;


pub struct RpcConfig {
    pub listen_address: IpAddr,
    pub port: u16,
}

impl TryFrom<PartialRpcConfig> for RpcConfig {
    type Error = ConfigError;
    fn try_from(config: PartialRpcConfig) -> Result<Self, Self::Error> {
        Ok(Self{
            listen_address: config.listen_address.ok_or(ConfigError::MissingConfig("listen_address".to_string()))?,
            port: config.port.ok_or(ConfigError::MissingConfig("port".to_string()))?,
        })
    }
}

#[derive(Args, Clone, Debug, Deserialize, Emptiable, Mergeable, Serialize)]
pub struct PartialRpcConfig {
    pub listen_address: Option<IpAddr>,
    pub port: Option<u16>,
}
impl PartialRpcConfig {
    pub fn with_unused_port(mut self) -> Self {
        let listneer = if let Some(x) = self.listen_address {
            TcpListener::bind(SocketAddr::new(x,0)).unwrap()
        } else {
            TcpListener::bind("127.0.0.1:0").unwrap()
        };
        self.port = Some(listneer.local_addr().unwrap().port());
        self
    }
}

impl Default for PartialRpcConfig {
    fn default() -> Self {
        Self{
            listen_address: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            port: None,
        }
    }
}

impl From<RpcConfig> for PartialRpcConfig {
    fn from(source: RpcConfig) -> Self {
        Self {
            listen_address: Some(source.listen_address),
            port: Some(source.port),
        }
    }
}


