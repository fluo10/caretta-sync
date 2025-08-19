use caretta::{config::P2pConfig, server::ServerTrait};

pub struct Server{};

impl ServerTrait for Server {
    async fn serve_p2p(config: P2pConfig) -> Result<(), caretta::error::Error> {
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(self.secret)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|keypair| p2p::Behaviour::try_from(keypair).unwrap())?
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
}