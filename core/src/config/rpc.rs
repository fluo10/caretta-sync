use crate::utils::{emptiable::Emptiable, mergeable::Mergeable};
#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::error::ConfigError;

#[cfg(unix)]
static DEFAULT_PORT: u16 = 54321;

#[derive(Clone, Debug)]
pub struct RpcConfig {
    pub endpoint_url: Url,
}

impl TryFrom<PartialRpcConfig> for RpcConfig {
    type Error = ConfigError;
    fn try_from(config: PartialRpcConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            endpoint_url: config
                .endpoint_url
                .ok_or(ConfigError::MissingConfig("rpc.endpoint"))?,
        })
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialRpcConfig {
    pub endpoint_url: Option<Url>,
}

impl PartialRpcConfig {
    #[cfg(not(any(all(target_os = "ios", target_abi = "sim"), target_os = "windows")))]
    pub fn default(app_name: &'static str) -> Self {
        let username = whoami::username();
        Self {
            endpoint_url: Some(
                Url::parse(
                    &(String::from("unix://")
                        + std::env::temp_dir()
                            .join(username)
                            .join(String::from(app_name) + ".sock")
                            .to_str()
                            .unwrap()),
                )
                .unwrap(),
            ),
        }
    }
    #[cfg(any(all(target_os = "ios", target_abi = "sim"), target_os = "windows"))]
    pub fn default(app_name: &'static str) -> Self {
        Self {
            endpoint_url: Some(Url::parse("http://127.0.0.1:54321").unwrap()),
        }
    }
}

impl Emptiable for PartialRpcConfig {
    fn empty() -> Self {
        Self { endpoint_url: None }
    }
    fn is_empty(&self) -> bool {
        self.endpoint_url.is_none()
    }
}

impl From<RpcConfig> for PartialRpcConfig {
    fn from(source: RpcConfig) -> Self {
        Self {
            endpoint_url: Some(source.endpoint_url),
        }
    }
}

impl Mergeable for PartialRpcConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.endpoint_url {
            self.endpoint_url = Some(x);
        }
    }
}

impl Mergeable for Option<PartialRpcConfig> {
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
