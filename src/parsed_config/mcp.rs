use std::{net::{IpAddr, SocketAddr}, path::PathBuf};

use chrono::format::Parsed;
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::parsed_config::error::ParsedConfigError;
use crate::{
    config::{McpConfig,},
    util::{Emptiable, Mergeable},
};

impl TryFrom<ParsedMcpConfig> for McpConfig {
    type Error = ParsedConfigError;
    fn try_from(value: ParsedMcpConfig) -> Result<Self, Self::Error> {
        let endpoint_url =  value.endpoint_url.ok_or(ParsedConfigError::MissingConfig("endpoint_url"))?; 
        let listen_addr = value.listen_addr.ok_or(ParsedConfigError::MissingConfig("listen_addr"))?;
        Ok(Self {
            endpoint_url, listen_addr, access_token: value.access_token
        })

    }
}

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ParsedMcpConfig {
    #[arg(long = "mcp-endpoint-url", env = "MCP_ENDPOINT_URL")]
    pub endpoint_url: Option<String>,
    #[arg(long = "mcp-listen-addr", env = "MCP_LISTEN_ADDR")]
    pub listen_addr: Option<SocketAddr>,
    #[arg(long = "mcp-access-token", env = "MCP_ACCESS_TOKEN")]
    pub access_token: Option<String>,
}

impl ParsedMcpConfig {
    pub fn default(app_name: &'static str) -> Self {
        McpConfig::default(app_name).into()
    }
}

impl Emptiable for ParsedMcpConfig {
    fn empty() -> Self {
        Self { 
            endpoint_url: None,
            listen_addr: None,
            access_token: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.endpoint_url.is_none()
        && self.listen_addr.is_none()
        && self.access_token.is_none()
    }
}

impl From<McpConfig> for ParsedMcpConfig {
    fn from(value: McpConfig) -> Self {
        Self {
            endpoint_url: Some(value.endpoint_url),
            listen_addr: Some(value.listen_addr),
            access_token: value.access_token
        }
    }
}

impl Mergeable for ParsedMcpConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.endpoint_url {
            self.endpoint_url = Some(x);
        }
        if let Some(x) = other.listen_addr {
            self.listen_addr = Some(x);
        }
        if let Some(x) = other.access_token {
            self.access_token = Some(x);
        }

    }
}