use std::net::{Ipv4Addr, SocketAddrV4};

#[cfg(feature = "cli")]
use clap::Args;
use futures::StreamExt;
use iroh::{
    Endpoint, SecretKey,
    discovery::{dns::DnsDiscovery, mdns::MdnsDiscovery}, protocol::Router,
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::{
    config::ConfigError, error::Error, models::P2pConfigModel, utils::{emptiable::Emptiable, mergeable::Mergeable}
};

#[derive(Clone, Debug)]
pub struct P2pConfig {
    pub enabled: bool,
    pub secret_key: SecretKey,
    pub enable_mdns: bool,
    pub enable_n0: bool,
}

impl P2pConfig {
    pub async fn to_iroh_router(&self) -> Result<Option<Router>, crate::error::Error> {
        if self.enabled {
            let mut endpoint = iroh::endpoint::Builder::empty(iroh::RelayMode::Disabled)
                .secret_key(self.secret_key.clone());

            let endpoint_id = self.secret_key.public();
            if self.enable_n0 {
                endpoint = endpoint.discovery(DnsDiscovery::n0_dns().build());
            }
            if self.enable_mdns {
                endpoint = endpoint.discovery(MdnsDiscovery::builder().build(endpoint_id).unwrap());
            }
            let endpoint = endpoint.bind().await?;
            Ok(Some(Router::builder(endpoint)
                .accept(iroh_ping::ALPN, iroh_ping::Ping::new())
                .spawn()))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<PartialP2pConfig> for P2pConfig {
    type Error = ConfigError;
    fn try_from(raw: PartialP2pConfig) -> Result<P2pConfig, Self::Error> {
        Ok(P2pConfig {
            enabled: raw.enabled.ok_or(ConfigError::MissingConfig("p2p.enabled"))?,
            secret_key: raw
                .secret_key
                .ok_or(ConfigError::MissingConfig("p2p.secret_key"))?,
            enable_n0: raw
                .enable_n0
                .ok_or(ConfigError::MissingConfig("p2p.enable_n0"))?,
            enable_mdns: raw
                .enable_mdns
                .ok_or(ConfigError::MissingConfig("p2p.enable_mdns"))?,
        })
    }
}

impl From<P2pConfigModel> for P2pConfig {
    fn from(value: P2pConfigModel) -> Self {
        Self {
            enabled: value.enabled,
            secret_key: value.secret_key.into(),
            enable_mdns: value.enable_mdns,
            enable_n0: value.enable_n0,
        }
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialP2pConfig {
    #[cfg_attr(feature = "cli", arg(long = "p2p_enable"))]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(long))]
    pub secret_key: Option<SecretKey>,
    #[cfg_attr(feature = "cli", arg(long))]
    pub enable_n0: Option<bool>,
    #[cfg_attr(feature = "cli", arg(long))]
    pub enable_mdns: Option<bool>,
}

impl From<P2pConfig> for PartialP2pConfig {
    fn from(config: P2pConfig) -> Self {
        Self {
            enabled: Some(config.enabled),
            secret_key: Some(config.secret_key),
            enable_mdns: Some(config.enable_mdns),
            enable_n0: Some(config.enable_n0)
        }
    }
}

impl Emptiable for PartialP2pConfig {
    fn empty() -> Self {
        Self {
            enabled: None,
            secret_key: None,
            enable_mdns: None,
            enable_n0: None
        }
    }

    fn is_empty(&self) -> bool {
        self.enabled.is_none()
            && self.secret_key.is_none()
            && self.enable_mdns.is_none()
            && self.enable_n0.is_none()
    }
}

impl Mergeable for PartialP2pConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.enabled.take() {
            let _ = self.enabled.insert(x);
        };
        if let Some(x) = other.secret_key.take() {
            let _ = self.secret_key.insert(x);
        };
        if let Some(x) = other.enable_n0.take() {
            let _ = self.enable_n0.insert(x);
        };
        if let Some(x) = other.enable_mdns.take() {
            let _ = self.enable_mdns.insert(x);
        };
    }
}
impl Mergeable for Option<PartialP2pConfig> {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.take() {
            if let Some(y) = self.as_mut() {
                y.merge(x);
            } else {
                let _ = self.insert(x);
            }
        };
    }
}
