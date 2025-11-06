use std::{convert::Infallible, marker::PhantomData, path::PathBuf, sync::Arc};

use http::{Request};
use iroh::discovery::mdns::MdnsDiscovery;
use sea_orm_migration::MigratorTrait;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::{body::Body, server::NamedService};
use tower_layer::Identity;
use tower_service::Service;

use crate::{context::ServerContext, error::Error, proto::api::{device::{DeviceServer, device_service_server::DeviceServiceServer}, invitation_token::{InvitationTokenServer, invitation_token_service_server::InvitationTokenServiceServer}}};


#[async_trait::async_trait]
pub trait ServerTrait {
    async fn serve(context: ServerContext) -> Result<(), Error>;
}

pub struct Server

{
    tonic_router: tonic::transport::server::Router<Identity>,
    context: Arc<ServerContext>,
}

impl Server 
{
    pub fn new(context: ServerContext) -> Self {
        let context = Arc::new(context);
        Self {
            context: context.clone(),
            tonic_router: tonic::transport::Server::builder()
                .add_service(DeviceServiceServer::new(DeviceServer::new(context.clone())))
                .add_service(InvitationTokenServiceServer::new(InvitationTokenServer::new(context.clone())))
        }
    }
    pub fn add_service<S>(mut self, svc: S) -> Self
    where
        S: Service<Request<Body>, Error = Infallible> + NamedService + Clone + Send + Sync + 'static,
        S::Response: axum::response::IntoResponse,
        S::Future: Send + 'static,
    {
        self.tonic_router = self.tonic_router.add_service(svc);
        self
    }
    pub async fn serve(self) -> Result<(), Error> {
        let url = self.context.as_ref().rpc_config.endpoint_url.clone();
        let rpc = tokio::spawn(async move  {

            match url.scheme() {
                "unix" => {
                    let path = PathBuf::from(url.path());
                    if let Some(x) = path.parent() {
                        if !x.exists() {
                            std::fs::create_dir_all(x)
                            .expect("Failed to create directory for socket file!");
                        }
                    }
                    if path.exists() {
                        std::fs::remove_file(&path).expect("Failed to remove existing socket file!")
                    }
                    let uds = UnixListener::bind(path).unwrap();
                    let uds_stream = UnixListenerStream::new(uds);
                    
                    self.tonic_router.serve_with_incoming(uds_stream).await.unwrap();
                }
                "http" => {
                    let host = url
                        .socket_addrs(|| None)
                        .expect("http endpoint should have host address and port")
                        .pop()
                        .unwrap();
                    self.tonic_router.serve(host).await.unwrap();
                }
                _ => {
                    panic!("Invalid url scheme!")
                }
            }
        });
        rpc.await;
        Ok(())
    }
}
