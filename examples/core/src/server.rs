use std::path::PathBuf;

use caretta_sync::{
    config::IrohConfig,
    global::IROH_ENDPOINT,
    proto::api::{
        device::{DeviceServer, device_service_server::DeviceServiceServer},
        device_verification::{
            DeviceVerificationServier,
            device_verification_service_server::DeviceVerificationServiceServer,
        },
    },
    server::ServerTrait,
};
use iroh::discovery::dns::DnsDiscovery;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::async_trait;

#[derive(Debug)]
pub struct Server;

#[async_trait]
impl ServerTrait for Server {
    async fn serve_rpc<T>(config: &T) -> Result<(), caretta_sync::error::Error>
    where
        T: AsRef<caretta_sync::config::RpcConfig> + Send + Sync,
    {
        let url = config.as_ref().endpoint_url.clone();
        let router = tonic::transport::Server::builder()
            .add_service(DeviceServiceServer::new(DeviceServer))
            .add_service(DeviceVerificationServiceServer::new(
                DeviceVerificationServier,
            ));
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

                router.serve_with_incoming(uds_stream).await.unwrap();
            }
            "http" => {
                let host = url
                    .socket_addrs(|| None)
                    .expect("http endpoint should have host address and port")
                    .pop()
                    .unwrap();

                router.serve(host).await.unwrap();
            }
            _ => {
                Err(caretta_sync::error::Error::Config(
                    caretta_sync::config::ConfigError::InvalidUrl(url),
                ))?;
            }
        }
        Ok(())
    }
}
