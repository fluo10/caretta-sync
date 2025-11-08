use std::{marker::PhantomData, pin::Pin};

use futures::{Stream, StreamExt};
use iroh::{
    Endpoint,
    discovery::{ConcurrentDiscovery, Discovery, DiscoveryError, DiscoveryItem},
    protocol::Router,
};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::{
    config::{LogConfig, P2pConfig, RpcConfig, StorageConfig},
    error::Error,
};

#[derive(Clone, Debug)]
pub struct BackendContext {
    pub app_name: &'static str,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_router: Option<Router>,
}
impl BackendContext {
    pub fn as_iroh_router(&self) -> Option<&Router> {
        self.iroh_router.as_ref()
    }
    pub fn as_endpoint(&self) -> Option<&Endpoint> {
        self.as_iroh_router().map(|x| x.endpoint())
    }
    pub fn as_discovery(&self) -> Option<&ConcurrentDiscovery> {
        self.as_endpoint().map(|x| x.discovery())
    }
    pub async fn discover(
        &self,
        endpoint_id: iroh::EndpointId,
    ) -> Option<
        Pin<
            Box<
                dyn Stream<Item = Result<DiscoveryItem, DiscoveryError>>
                    + std::marker::Send
                    + 'static,
            >,
        >,
    > {
        if let Some(x) = self.as_discovery() {
            x.resolve(endpoint_id)
        } else {
            None
        }
    }
}

impl AsRef<DatabaseConnection> for BackendContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}
impl From<&BackendContext> for Option<Endpoint> {
    fn from(value: &BackendContext) -> Self {
        value.iroh_router.as_ref().map(|x| x.endpoint().clone())
    }
}
impl From<&BackendContext> for Option<ConcurrentDiscovery> {
    fn from(value: &BackendContext) -> Self {
        value
            .iroh_router
            .as_ref()
            .map(|x| x.endpoint().discovery().clone())
    }
}
