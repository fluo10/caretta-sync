use std::sync::Arc;

use iroh::protocol::Router;
use irpc::util::make_server_endpoint;
#[cfg(feature = "desktop")]
use n0_future::task::AbortOnDropHandle;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::{config::{IpcConfig, LogConfig, P2pConfig, StorageConfig}, ipc::{IpcActor, IpcContext}};

pub struct Engine{
    iroh_router: Router,
    #[cfg(feature = "desktop")]
    ipc_server: AbortOnDropHandle<()>,
    
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub async fn wait_shutdown(self) -> Result<(), n0_future::task::JoinError> {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Receive ctrl-c event.");
        self.shutdown().await
    }

    async fn shutdown(self) -> Result<(), n0_future::task::JoinError>{
        self.iroh_router.shutdown().await
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

        let (server, cert) = make_server_endpoint(self.ipc_config.unwrap().endpoint.clone()).unwrap();
        let actor = IpcActor::spawn(IpcContext {
            database_connection: self.database_connection.unwrap().clone(),
            iroh_endpoint: self.p2p_config.unwrap().spawn_iroh_endpoint().await.unwrap(),
            docs: docs.api().clone()
        });
        let handle = actor.listen(server).unwrap();
        Engine {
            iroh_router: router,
            ipc_server: handle
        }
    }
}
