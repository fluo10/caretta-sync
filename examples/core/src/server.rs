use std::path::PathBuf;

use caretta_sync::{
    config::P2pConfig,
    proto::cached_peer_service_server::CachedPeerServiceServer,
    server::ServerTrait,
    rpc::service::iroh::CachedPeerService
};
use libp2p::{futures::StreamExt, noise, swarm::SwarmEvent, tcp, yamux};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;

#[derive(Debug)]
pub struct Server{}

impl ServerTrait for Server {
    async fn serve_p2p<T>(config: &T) -> Result<(), caretta_sync::error::Error>
    where 
        T: AsRef<P2pConfig> 
    {
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.as_ref().private_key.clone())
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|keypair| caretta_sync::p2p::Behaviour::try_from(keypair).unwrap())?
            .build();
            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        loop{
            let swarm_event = swarm.select_next_some().await;
            tokio::spawn(async move{
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => {
                        println!("{event:?}");
                        event.run().await;
                    },
                    _ => {}
                }
            });
        }
    }
    
    async fn serve_rpc<T>(config: &T) -> Result<(), caretta_sync::error::Error>
    where T: AsRef<caretta_sync::config::RpcConfig> {
        let url = config.as_ref().endpoint_url.clone();
        let router =  tonic::transport::Server::builder()
            .add_service(CachedPeerServiceServer::new(CachedPeerService::default()));
        match url.scheme() {
            "unix" => {
                let path = PathBuf::from(url.path());
                if let Some(x) = path.parent() {
                    if !x.exists() {
                        std::fs::create_dir_all(x).expect("Failed to create directory for socket file!");
                    }
                }
                if path.exists() {
                    std::fs::remove_file(&path).expect("Failed to remove existing socket file!")
                }
                let uds = UnixListener::bind(path).unwrap();
                let uds_stream = UnixListenerStream::new(uds);

                router.serve_with_incoming(uds_stream)
                    .await.unwrap();
            },
            "http" => {
                let host = url.socket_addrs(|| None).expect("http endpoint should have host address and port").pop().unwrap();

                router.serve(host).await.unwrap();
            },
            _ => {
                Err(caretta_sync::error::Error::Config(caretta_sync::config::ConfigError::InvalidUrl(url)))?;
            }
        }
        Ok(())
    }
}