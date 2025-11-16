use std::{convert::Infallible, marker::PhantomData, path::PathBuf, sync::Arc};

use http::Request;
use iroh::discovery::mdns::MdnsDiscovery;
use sea_orm_migration::MigratorTrait;
use tokio::net::UnixListener;
use tower_layer::Identity;
use tower_service::Service;

use caretta_sync_core::{
    context::{ServerContext, ServiceContext},
};

use crate::{
    error::ServiceError,
};

#[async_trait::async_trait]
pub trait ServerTrait {
    async fn serve(context: ServerContext) -> Result<(), ServiceError>;
}

pub struct Server {
    context: Arc<ServerContext>,
}

impl Server {
    pub fn new(context: ServerContext) -> Self {
        let context = Arc::new(context);
        let backend_context: Arc<dyn AsRef<ServiceContext> + Send + Sync> = context.clone();

        Self {
            context: context.clone(),
        }
    }
    pub async fn serve(self) -> Result<(), ServiceError> {
        todo!();
        Ok(())
    }
}
