use std::{marker::PhantomData, pin::Pin, sync::Arc};

use futures::{Stream, StreamExt};
use iroh::{
    Endpoint,
    discovery::{ConcurrentDiscovery, Discovery, DiscoveryError, DiscoveryItem},
    protocol::Router,
};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::{
    config::{LogConfig, P2pConfig, RpcConfig, StorageConfig}, context::{ServiceContext, service::ServiceContextExt}, error::Error
};

#[derive(Clone, Debug)]
pub struct ServerContext {
    pub app_name: &'static str,
    pub rpc_config: RpcConfig,
    pub service_context: ServiceContext
}

impl AsRef<ServiceContext> for ServerContext {
    fn as_ref(&self) -> &ServiceContext {
        &self.service_context
    }
}