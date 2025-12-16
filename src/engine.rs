use std::sync::Arc;

use iroh::protocol::Router;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::{config::{IpcConfig, LogConfig, P2pConfig, StorageConfig}, };

pub struct Engine{
    iroh_router: Router,
    
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct EngineBuilder {
    ipc_config: Option<IpcConfig>,
    log_config: Option<LogConfig>,
    p2p_config: Option<P2pConfig>,
    storage_config: Option<StorageConfig>,
    database_connection: Option<Arc<DatabaseConnection>>,
}

impl EngineBuilder {
    pub fn ipc_config(mut self, config: IpcConfig) -> Self {
        self.ipc_config.insert(config);
        self
    }
    pub fn log_config(mut self, config: LogConfig) -> Self {
        self.log_config.insert(config);
        self
    }  
    pub fn p2p_config(mut self, config: P2pConfig) -> Self {
        self.p2p_config.insert(config);
        self
    }
    pub fn storage_config(mut self, config: StorageConfig) -> Self {
        self.storage_config.insert(config);
        self
    }
    pub fn load_db(mut self) -> Self {
        todo!()
    }
    pub async fn spawn(self) -> Engine{
        let (endpoint, blobs, docs , gossip) = self.p2p_config.clone().unwrap().spawn_iroh_protocols(&self.storage_config.unwrap()).await.unwrap();
        let router = Router::builder(endpoint)
            .accept(&iroh_blobs::ALPN, blobs)
            .accept(&iroh_ping::ALPN, iroh_ping::Ping::new())
            .accept(&iroh_gossip::ALPN, gossip)
            .accept(&iroh_docs::ALPN, docs.clone())
            .spawn();


        todo!()
    }
}
