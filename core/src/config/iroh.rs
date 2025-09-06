use std::{net::{IpAddr, Ipv4Addr}, ops, path::{Path, PathBuf}};

use base64::{prelude::BASE64_STANDARD, Engine};
#[cfg(feature="cli")]
use clap::Args;
use futures::StreamExt;
use iroh::{Endpoint, SecretKey};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing_subscriber::EnvFilter;


use crate::{
    config::PartialConfig,
    error::Error, utils::{emptiable::Emptiable, mergeable::Mergeable}
};

#[derive(Clone, Debug)]
pub struct IrohConfig {
    pub enable: bool,
    pub secret_key: SecretKey,
    pub use_n0_discovery_service: bool,
}

impl IrohConfig {
    async fn into_endpoint(config: Self) -> Result<Option<Endpoint>, crate::error::Error> {
        if config.enable {
            let mut endpoint = Endpoint::builder()
                .secret_key(config.secret_key)
                .discovery_dht()
                .discovery_local_network();
            if config.use_n0_discovery_service {
                endpoint = endpoint.discovery_n0();
            }
            Ok(Some(endpoint.bind().await?))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<PartialIrohConfig> for IrohConfig {
    type Error = crate::error::Error;
    fn try_from(raw: PartialIrohConfig) -> Result<IrohConfig, Self::Error> {
        Ok(IrohConfig {
            enable: raw.enable.ok_or(Error::MissingConfig("iroh.enable"))?,
            secret_key: raw.secret_key.ok_or(Error::MissingConfig("iroh.secret_key"))?,
            use_n0_discovery_service: raw.use_n0_discovery_service.ok_or(Error::MissingConfig("iroh.use_n0_discovery_service"))?
        })
    }
}



#[cfg_attr(feature="cli",derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialIrohConfig {
    #[cfg_attr(feature="cli",arg(long="p2p_enable"))]
    pub enable: Option<bool>,
    #[cfg_attr(feature="cli",arg(long))]
    pub secret_key: Option<SecretKey>,
    #[cfg_attr(feature="cli",arg(long))]
    pub use_n0_discovery_service: Option<bool>,
}

impl PartialIrohConfig {
    pub fn with_new_secret_key(mut self) -> Self {
        let mut rng = rand::rngs::OsRng;
        self.secret_key = Some(SecretKey::generate(&mut rng));
        self
    }
}

impl From<IrohConfig> for PartialIrohConfig {
    fn from(config: IrohConfig) -> Self {
        Self {
            enable: Some(config.enable),
            secret_key: Some(config.secret_key),
            use_n0_discovery_service: Some(config.use_n0_discovery_service)
        }
    }
}

impl Default for PartialIrohConfig {
    fn default() -> Self {
        Self {
            enable: Some(true),
            secret_key: None,
            use_n0_discovery_service: Some(true)
        }
    }
}

impl Emptiable for PartialIrohConfig {
    fn empty() -> Self {
        Self{
            enable: None,
            secret_key: None,
            use_n0_discovery_service: None
        }
    }

    fn is_empty(&self) -> bool {
        self.enable.is_none() && self.secret_key.is_none() && self.use_n0_discovery_service.is_none()
    }
}

impl Mergeable for PartialIrohConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.enable.take() {
            let _ = self.enable.insert(x);
        };
        if let Some(x) = other.secret_key.take() {
            let _ = self.secret_key.insert(x);
        };
        if let Some(x) = other.use_n0_discovery_service.take() {
            let _ = self.use_n0_discovery_service.insert(x);
        };
    }
}
impl Mergeable for Option<PartialIrohConfig> {
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
