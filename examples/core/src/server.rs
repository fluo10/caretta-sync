use caretta::{
    config::P2pConfig,
    proto::cached_peer_service_server::CachedPeerServiceServer,
    server::ServerTrait,
    rpc::service::cached_peer::CachedPeerService
};
use libp2p::{futures::StreamExt, noise, swarm::SwarmEvent, tcp, yamux};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
pub struct Server{}

impl ServerTrait for Server {
    async fn serve_p2p<T>(config: &T) -> Result<(), caretta::error::Error>
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
            .with_behaviour(|keypair| caretta::p2p::Behaviour::try_from(keypair).unwrap())?
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
    
    async fn serve_rpc<T>(config: &T) -> Result<(), caretta::error::Error>
    where T: AsRef<caretta::config::RpcConfig> {
        let path = config.as_ref().socket_path.clone();
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
        tonic::transport::Server::builder()
            .add_service(CachedPeerServiceServer::new(CachedPeerService::default()))
            .serve_with_incoming(uds_stream)
            .await.unwrap();
        Ok(())
    }
}