use caretta::{config::P2pConfig, server::ServerTrait};
use libp2p::{futures::StreamExt, noise, swarm::SwarmEvent, tcp, yamux};
pub struct Server{}

impl ServerTrait for Server {
    async fn serve_p2p<T>(config: &T) -> Result<(), caretta::error::Error>
    where 
        T: AsRef<P2pConfig> 
    {
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.as_ref().secret.clone())
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
        todo!()
    }
}