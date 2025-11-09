use std::{marker::PhantomData, pin::Pin};

use futures::{Stream, StreamExt};
use iroh::{
    Endpoint,
    discovery::{ConcurrentDiscovery, Discovery, DiscoveryError, DiscoveryItem},
    protocol::Router,
};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::config::{LogConfig, P2pConfig, RpcConfig, StorageConfig};

/// An extension trait for [`ServiceContext`]
pub trait ServiceContextExt {
    fn as_iroh_router(&self) -> Option<&Router>;
    fn as_endpoint(&self) -> Option<&Endpoint> {
        self.as_iroh_router().map(|x| x.endpoint())
    }
    fn as_discovery(&self) -> Option<&ConcurrentDiscovery> {
        self.as_endpoint().map(|x| x.discovery())
    }
    async fn discover(
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

impl<T> ServiceContextExt for T
where
    T: AsRef<ServiceContext>,
{
    fn as_iroh_router(&self) -> Option<&Router> {
        self.as_ref().as_iroh_router()
    }
}

/// A context for background process
#[derive(Clone, Debug)]
pub struct ServiceContext {
    pub app_name: &'static str,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_router: Option<Router>,
}
impl ServiceContextExt for ServiceContext {
    fn as_iroh_router(&self) -> Option<&Router> {
        self.iroh_router.as_ref()
    }
}

impl AsRef<DatabaseConnection> for ServiceContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}
impl From<&ServiceContext> for Option<Endpoint> {
    fn from(value: &ServiceContext) -> Self {
        value.iroh_router.as_ref().map(|x| x.endpoint().clone())
    }
}
impl From<&ServiceContext> for Option<ConcurrentDiscovery> {
    fn from(value: &ServiceContext) -> Self {
        value
            .iroh_router
            .as_ref()
            .map(|x| x.endpoint().discovery().clone())
    }
}
