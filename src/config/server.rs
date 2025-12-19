use std::pin::Pin;

use iroh_docs::protocol::Docs;
use rmcp::{RoleServer, Service};

use crate::{config::{LogConfig, McpConfig, P2pConfig, ServerConfigExt, StorageConfig}, server::Server};

/// A config for server for desktop OS
#[derive(Clone, Debug)]
pub struct ServerConfig {
    log: LogConfig,
    mcp: McpConfig,
    p2p: P2pConfig,
    storage: StorageConfig,
}

impl ServerConfig {
    pub async fn spawn_server<S,M> (self, app_name: &'static str,  service_factory: impl Fn(ServerConfig, Docs ) -> Result<S, std::io::Error> + Send + Sync + 'static)
    where 
    S: Service<RoleServer> + Send + 'static,
    M: rmcp::transport::streamable_http_server::SessionManager
    {
        use rmcp::transport::{StreamableHttpServerConfig, StreamableHttpService, streamable_http_server::session::local::LocalSessionManager};
        let mcp_config = &self.mcp;
        let p2p_config = &self.p2p;
        let storage_config = &self.storage;
        let (irou_endpoint, iroh_docs, iroh_router_builder) = self.to_iroh_router_builder(app_name).await.unwrap();
        let database_connection = storage_config.to_database_connection().await;
        let ct = tokio_util::sync::CancellationToken::new();
        let config = Box::pin(self.clone());
        let docs = Box::pin(iroh_docs);
        let service = StreamableHttpService::new(
              move || {service_factory((*config).clone(), (*docs).clone())},
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig {
                cancellation_token: ct.child_token(),
                ..Default::default()
            },
        );
        let router = axum::Router::new().nest_service("/", service);
        let tcp_listener = mcp_config.bind_tcp_listener().await;
        let p2p_handler = tokio::spawn(async move {
            let router = iroh_router_builder.spawn();
            tokio::signal::ctrl_c().await.unwrap();
            router.shutdown().await.unwrap();
        });
        let mcp_handler = tokio::spawn(async move {
            axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async move {
                tokio::signal::ctrl_c().await.unwrap();
                ct.cancel();
            }).await.unwrap()
        });
        let _ = tokio::try_join!(p2p_handler, mcp_handler).unwrap();
    }
}
impl ServerConfigExt for ServerConfig {
    fn p2p_config(&self) -> &P2pConfig {
        &self.p2p
    }

    fn storage_config(&self) -> &StorageConfig {
        &self.storage
    }

    
}