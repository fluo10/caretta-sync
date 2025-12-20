use std::{marker::PhantomData, pin::Pin, sync::{Arc, OnceLock}};

use iroh_docs::protocol::Docs;
use rmcp::{RoleServer, Service};
use sea_orm_migration::MigratorTrait;

use crate::{config::{LogConfig, McpConfig, P2pConfig, ServerConfigExt, StorageConfig}, mcp::context::McpContext};

/// A config for server for desktop OS
#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub log: LogConfig,
    pub mcp: McpConfig,
    pub p2p: P2pConfig,
    pub storage: StorageConfig,
}

static CONTEXT: OnceLock<McpContext> = OnceLock::new();

impl ServerConfig {
    pub async fn spawn_server<S,M> (self, app_name: &'static str)
    where 
    S: Service<RoleServer> + From<&'static McpContext> + Send + 'static,
    M: MigratorTrait
    {
        use rmcp::transport::{StreamableHttpServerConfig, StreamableHttpService, streamable_http_server::session::local::LocalSessionManager};
        let mcp_config = &self.mcp;
        let p2p_config = &self.p2p;
        let storage_config = &self.storage;
        let (iroh_endpoint, iroh_docs, iroh_router_builder) = self.to_iroh_router_builder(app_name).await.unwrap();
        let database = storage_config.open_database().await;
        let ct = tokio_util::sync::CancellationToken::new();
        CONTEXT.set(McpContext {
            app_database: storage_config.open_app_database::<M>(app_name,).await,
            database: database,
            iroh_endpoint: iroh_endpoint,
            docs: iroh_docs.api().clone(),
        }).unwrap();
        let service = StreamableHttpService::new(
               || Ok(S::from((CONTEXT.get().unwrap()))),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig {
                cancellation_token: ct.child_token(),
                ..Default::default()
            },
        );
        let router = axum::Router::new().fallback_service(service);
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