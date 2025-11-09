use std::path::PathBuf;

use caretta_sync::{
    context::ServerContext,
    error::Error,
    proto::api::device::{DeviceServer, device_service_server::DeviceServiceServer},
    server::ServerTrait,
};
use iroh::discovery::dns::DnsDiscovery;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;

#[derive(Debug)]
pub struct Server;

#[async_trait::async_trait]
impl ServerTrait for Server {
    async fn serve(context: ServerContext) -> Result<(), Error> {
        caretta_sync::core::server::Server::new(context)
            .serve()
            .await
    }
}
