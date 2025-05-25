use std::{collections::HashSet, net::{IpAddr, Ipv4Addr}, str::FromStr, sync::LazyLock};

use clap::Args;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub static DEFAULT_LISTEN_IPS: &[IpAddr] = &[IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
pub static DEFAULT_PORT: u16 = 8080;
pub static DEFAULT_SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| {
    ServerConfig{
        listen_ips: Vec::from(DEFAULT_LISTEN_IPS),
        port: DEFAULT_PORT
    }
});
pub static DEFAULT_PARTIAL_SERVER_CONFIG: LazyLock<PartialServerConfig> = LazyLock::new(|| {
    PartialServerConfig::from((*DEFAULT_SERVER_CONFIG).clone())
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    listen_ips: Vec<IpAddr>,
    port: u16,
}

impl TryFrom<PartialServerConfig> for ServerConfig {
    type Error = Error;
    fn try_from(config: PartialServerConfig) -> Result<ServerConfig, Self::Error>{
        Ok(ServerConfig {
            listen_ips: config.listen_ips.ok_or(Error::MissingConfig("listen_ips".to_string()))?,
            port: config.port.ok_or(Error::MissingConfig("port".to_string()))?
        })
    }
}

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct PartialServerConfig {
    #[arg(long)]
    listen_ips: Option<Vec<IpAddr>>,
    #[arg(long)]
    port: Option<u16>,
}

impl From<ServerConfig> for PartialServerConfig {
    fn from(config: ServerConfig) -> PartialServerConfig {
        PartialServerConfig {
            listen_ips: Some(config.listen_ips),
            port: Some(config.port)
        }
    }
}



