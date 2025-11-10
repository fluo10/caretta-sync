use sea_orm::DatabaseConnection;
use tonic::transport::Endpoint;

use crate::{
    config::{RpcConfig},
    error::CoreError,
};

/// A context for client
#[derive(Clone, Debug)]
pub struct ClientContext {
    pub app_name: &'static str,
    pub rpc_config: RpcConfig,
}

impl AsRef<ClientContext> for ClientContext {
    fn as_ref(&self) -> &ClientContext {
        self
    }
}

impl TryFrom<&ClientContext> for Endpoint {
    type Error = CoreError;
    fn try_from(value: &ClientContext) -> Result<Self, Self::Error> {
        Ok(value.rpc_config.endpoint_url.to_string().try_into()?)
    }
}
