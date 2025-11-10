use clap::Args;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::parsed_config::error::ParsedConfigError;
use caretta_sync_core::{
    config::RpcConfig,
    util::{Emptiable, Mergeable},
};

impl TryFrom<ParsedRpcConfig> for RpcConfig {
    type Error = ParsedConfigError;
    fn try_from(config: ParsedRpcConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            endpoint_url: config
                .endpoint_url
                .ok_or(ParsedConfigError::MissingConfig("rpc.endpoint"))?,
        })
    }
}

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ParsedRpcConfig {
    #[arg(long = "rpc-endpoint-url", env = "RPC_ENDPOINT_URL")]
    pub endpoint_url: Option<Url>,
}

impl ParsedRpcConfig {
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

impl Emptiable for ParsedRpcConfig {
    fn empty() -> Self {
        Self { endpoint_url: None }
    }
    fn is_empty(&self) -> bool {
        self.endpoint_url.is_none()
    }
}

impl From<RpcConfig> for ParsedRpcConfig {
    fn from(source: RpcConfig) -> Self {
        Self {
            endpoint_url: Some(source.endpoint_url),
        }
    }
}

impl Mergeable for ParsedRpcConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.endpoint_url {
            self.endpoint_url = Some(x);
        }
    }
}
