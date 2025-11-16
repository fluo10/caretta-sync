use sea_orm::DatabaseConnection;

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
