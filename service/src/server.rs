use std::{convert::Infallible, marker::PhantomData, path::PathBuf, sync::Arc};

use iroh::discovery::mdns::MdnsDiscovery;
use irpc::util::make_server_endpoint;
use tokio::net::UnixListener;

use caretta_sync_core::{
    context::{ServerContext, ServiceContext}
};

use crate::{
    error::ServiceError, ipc::IpcActor,
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
    pub async fn serve(self) {
        let (server, cert) = make_server_endpoint(self.context.ipc_config.endpoint.clone()).unwrap();
        let context: Arc<dyn AsRef<ServiceContext> + Send + Sync> = self.context;
        let actor = IpcActor::spawn(&context);
        let handle = actor.listen(server).unwrap();
        handle.await;
    }
}
