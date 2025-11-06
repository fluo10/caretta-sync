use clap::Args;
use iroh::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};

use crate::{config::P2pConfig, parsed_config::error::ParsedConfigError, utils::{emptiable::Emptiable, mergeable::Mergeable}};

impl TryFrom<ParsedP2pConfig> for P2pConfig {
    type Error = ParsedConfigError;
    fn try_from(raw: ParsedP2pConfig) -> Result<P2pConfig, Self::Error> {
        Ok(P2pConfig {
            enabled: raw.enabled.ok_or(ParsedConfigError::MissingConfig("p2p.enabled"))?,
            secret_key: raw
                .secret_key
                .ok_or(ParsedConfigError::MissingConfig("p2p.secret_key"))?,
            enable_n0: raw
                .enable_n0
                .ok_or(ParsedConfigError::MissingConfig("p2p.enable_n0"))?,
            enable_mdns: raw
                .enable_mdns
                .ok_or(ParsedConfigError::MissingConfig("p2p.enable_mdns"))?,
        })
    }
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct ParsedP2pConfig {
    #[arg(long = "p2p_enable")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing)]
    #[arg(long)]
    pub secret_key: Option<SecretKey>,
    #[serde(skip_deserializing)]
    #[arg(skip)]
    pub public_key: Option<PublicKey>,
    #[arg(long)]
    pub enable_n0: Option<bool>,
    #[arg(long)]
    pub enable_mdns: Option<bool>,
}

impl From<P2pConfig> for ParsedP2pConfig {
    fn from(config: P2pConfig) -> Self {
        Self {
            enabled: Some(config.enabled),
            secret_key: Some(config.secret_key.clone()),
            public_key: Some(config.secret_key.public()),
            enable_mdns: Some(config.enable_mdns),
            enable_n0: Some(config.enable_n0)
        }
    }
}

impl Emptiable for ParsedP2pConfig {
    fn empty() -> Self {
        Self {
            enabled: None,
            secret_key: None,
            public_key: None,
            enable_mdns: None,
            enable_n0: None
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
impl Mergeable for Option<ParsedP2pConfig> {
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
