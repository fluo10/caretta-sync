use libp2p::{ identity::Keypair, mdns, ping, swarm};

use crate::error::Error;

#[derive(swarm::NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub struct Behaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,    
}

impl TryFrom<&Keypair> for Behaviour {
    type Error = Error;
    fn try_from(keypair: &Keypair) -> Result<Self, Error> {
        Ok(Self {
            mdns: mdns::tokio::Behaviour::new(
                mdns::Config::default(),
                keypair.public().into(),
            )?,
            ping: libp2p::ping::Behaviour::new(ping::Config::new()),
        })
    }
}

#[derive(Debug)]
pub enum Event {
    Mdns(mdns::Event),
    Ping(ping::Event),
}

impl Event {
    pub async fn run(self) {
        match self {
            Self::Mdns(x) => {
                match x {
                    mdns::Event::Discovered(e) => {
                        for peer in e {
                            let mut peers = crate::global::GLOBAL.write_peers().await;
                            peers.insert(peer.0, peer.1);
                        }
                        let peers = crate::global::GLOBAL.read_peers().await;
                        println!("Peers: {peers:?}");
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }
}
impl From<mdns::Event> for Event {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}
impl From<ping::Event> for Event {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}