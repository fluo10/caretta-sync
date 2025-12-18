use std::sync::Arc;

use iroh::protocol::Router;
use rmcp::{RoleServer, Service};
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::{config::{McpConfig, LogConfig, P2pConfig, StorageConfig}, };

pub struct Server{
    iroh_router: Router,
    
}

impl Server {

    pub fn builder() -> ServerBuilder {
        Server::default()
    }
}
