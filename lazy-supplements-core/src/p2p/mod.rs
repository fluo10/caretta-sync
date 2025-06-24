use libp2p::{ identity::Keypair, mdns, ping, swarm};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{cache::entity::{ActivePeerModel, PeerColumn, PeerEntity}, error::Error, global::{CACHE_DATABASE_CONNECTION, PEERS}};

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
    pub async fn run(&self)
    {
        match self {
            Self::Mdns(x) => {
                match x {
                    mdns::Event::Discovered(e) => {
                        for peer in e.iter() {
                            match PeerEntity::find().filter(PeerColumn::PeerId.contains(&peer.0.to_string())).one(CACHE_DATABASE_CONNECTION.get()).await {
                                Ok(_) => {}
                                Err(_) => {
                                    ActivePeerModel::new(peer.0.clone(), peer.1.clone())
                                        .insert(CACHE_DATABASE_CONNECTION.get()).await;
                                }
                            }
                        }
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