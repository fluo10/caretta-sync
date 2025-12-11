use std::{marker::PhantomData, pin::Pin, sync::Arc};

use futures::{Stream, StreamExt};
use iroh::{
    Endpoint,
    discovery::{ConcurrentDiscovery, Discovery, DiscoveryError, DiscoveryItem},
    protocol::Router,
};

use crate::{
    config::{LogConfig, P2pConfig, IpcConfig, StorageConfig},
    context::{ServiceContext, service::ServiceContextExt},
};

#[derive(Debug)]
pub struct ServerContext {
    pub app_name: &'static str,
    pub ipc_config: IpcConfig,
    pub service_context: ServiceContext,
}

impl AsRef<ServiceContext> for ServerContext {
    fn as_ref(&self) -> &ServiceContext {
        &self.service_context
    }
}
