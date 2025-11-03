use sea_orm::DatabaseConnection;
use tonic::transport::Endpoint;
use url::Url;

use crate::{config::{ConfigError, LogConfig, ParsedConfig, RpcConfig, StorageConfig}, error::Error};

/// A context for client
#[derive(Clone, Debug,)]
pub struct ClientContext {
    pub rpc_config: RpcConfig,
}

impl ClientContext {

    /// Convert context from [`ParsedConfig`]
    pub fn from_parsed_config<T>(config: T) -> Result<Self, ConfigError>
    where T: AsRef<ParsedConfig>
    {
        let  config= config.as_ref();
        let rpc_config = config.to_rpc_config()?;
        Ok(Self{rpc_config})
    }
}

impl AsRef<ClientContext> for ClientContext {
    fn as_ref(&self) -> &ClientContext {
        self
    }
}

impl TryFrom<&ClientContext> for Endpoint {
    type Error = Error;
    fn try_from(value: &ClientContext) -> Result<Self, Self::Error> {
        Ok(value.rpc_config.endpoint_url.to_string().try_into()?)
    }
}
