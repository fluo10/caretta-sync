use clap::Args;
use serde::{Deserialize, Serialize};

use crate::parsed_config::error::ParsedConfigError;
use caretta_sync_core::{
    serde::byte_array_option,
    util::{Emptiable, Mergeable},
};

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ParsedP2pConfig {
    #[arg(long = "p2p-enabled", env = "P2P_ENABLED")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing)]
    #[arg(long = "p2p-secret-key", env = "P2P_SECRET_KEY")]
    pub secret_key: Option<Vec<u8>>,
    #[serde(skip_deserializing)]
    #[arg(skip)]
    pub public_key: Option<Vec<u8>>,
    #[arg(long = "p2p-enable-n0", env = "P2P_ENABLE_N0")]
    pub enable_n0: Option<bool>,
    #[arg(long = "p2p-enable-mdns", env = "P2P_ENABLE_MDNS")]
    pub enable_mdns: Option<bool>,
}

impl Emptiable for ParsedP2pConfig {
    fn empty() -> Self {
        Self {
            enabled: None,
            secret_key: None,
            public_key: None,
            enable_mdns: None,
            enable_n0: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.enabled.is_none()
            && self.secret_key.is_none()
            && self.enable_mdns.is_none()
            && self.enable_n0.is_none()
            && self.public_key.is_none()
    }
}

impl Mergeable for ParsedP2pConfig {
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
        if let Some(x) = other.public_key.take() {
            let _ = self.public_key.insert(x);
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use std::array::TryFromSliceError;

    use super::*;
    use caretta_sync_core::config::P2pConfig;
    use iroh::SecretKey;
    impl TryFrom<ParsedP2pConfig> for P2pConfig {
        type Error = ParsedConfigError;
        fn try_from(raw: ParsedP2pConfig) -> Result<P2pConfig, Self::Error> {
            Ok(P2pConfig {
                enabled: raw
                    .enabled
                    .ok_or(ParsedConfigError::MissingConfig("p2p.enabled"))?,
                secret_key: raw
                    .secret_key
                    .ok_or(ParsedConfigError::MissingConfig("p2p.secret_key"))
                    .map(|x| {
                        let buf: [u8; 32] = x.as_slice().try_into()?;
                        Result::<SecretKey, TryFromSliceError>::Ok(SecretKey::from_bytes(&buf))
                    })??,
                enable_n0: raw
                    .enable_n0
                    .ok_or(ParsedConfigError::MissingConfig("p2p.enable_n0"))?,
                enable_mdns: raw
                    .enable_mdns
                    .ok_or(ParsedConfigError::MissingConfig("p2p.enable_mdns"))?,
            })
        }
    }
    impl From<P2pConfig> for ParsedP2pConfig {
        fn from(config: P2pConfig) -> Self {
            Self {
                enabled: Some(config.enabled),
                secret_key: Some(config.secret_key.to_bytes()[..].into()),
                public_key: Some(config.secret_key.public().as_bytes()[..].into()),
                enable_mdns: Some(config.enable_mdns),
                enable_n0: Some(config.enable_n0),
            }
        }
    }
}
