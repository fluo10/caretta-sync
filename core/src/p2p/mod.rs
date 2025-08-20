pub mod error;
use chrono::Local;
use libp2p::{ identity::Keypair, mdns, ping, swarm, Multiaddr, PeerId};
use sea_orm::{prelude::DateTimeUtc, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use tracing::{event, Level};

use crate::{cache::entity::{CachedPeerActiveModel, CachedAddressActiveModel, CachedAddressColumn, CachedAddressEntity, CachedAddressModel, CachedPeerColumn, CachedPeerEntity, CachedPeerModel}, data::value::{MultiaddrValue, PeerIdValue}, error::Error, global::DATABASE_CONNECTIONS};

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
                            event!(Level::TRACE, "Peer discovered via mdns: {}, {}", &peer.0, &peer.1);
                            match try_get_or_insert_cached_peer(&peer.0, &peer.1).await {
                                Ok(_) => {},
                                Err(e) => {
                                    event!(Level::WARN, "{:?}", e);
                                }
                            };
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

async fn try_get_or_insert_cached_peer(peer_id: &PeerId, peer_addr: &Multiaddr) -> Result<(CachedPeerModel, CachedAddressModel), Error> {
    match (
        CachedPeerEntity::find().filter(CachedPeerColumn::PeerId.eq(PeerIdValue::from(peer_id.clone()))).one(DATABASE_CONNECTIONS.get_cache_unchecked()).await?,
        CachedAddressEntity::find().filter(CachedAddressColumn::Multiaddress.eq(MultiaddrValue::from(peer_addr.clone()))).one(DATABASE_CONNECTIONS.get_cache_unchecked()).await?,
    ) {
        (Some(x), Some(y) ) => {
            if x.id == y.cached_peer_id {
                event!(Level::TRACE, "Known peer: {}, {}", peer_id, peer_addr);
                let mut addr: CachedAddressActiveModel = y.into();
                addr.updated_at = Set(Local::now().to_utc());
                let updated = addr.update(DATABASE_CONNECTIONS.get_cache_unchecked()).await?;
                Ok((x, updated))
            } else {
                y.delete(DATABASE_CONNECTIONS.get_cache().expect("Cache database should initialized beforehand!")).await?;
                Ok((x.clone(), CachedAddressActiveModel::new(x.id, peer_addr.clone()).insert(DATABASE_CONNECTIONS.get_cache_unchecked()).await?))
            }
        }
        (Some(x), None) => {
            event!(Level::INFO, "New address {} for {}", peer_addr, peer_id);
            Ok((x.clone(),CachedAddressActiveModel::new(x.id, peer_addr.clone()).insert(DATABASE_CONNECTIONS.get_cache_unchecked()).await?))
        },
        (None, x) =>  {
            event!(Level::INFO, "Add new peer: {}", peer_id);
            let inserted = CachedPeerActiveModel::new(peer_id.clone()).insert(DATABASE_CONNECTIONS.get_cache_unchecked()).await?;
            if let Some(y) = x {
                event!(Level::INFO, "Remove {} from {}", peer_addr, peer_id);
                y.delete(DATABASE_CONNECTIONS.get_cache_unchecked()).await?;
            };
            event!(Level::INFO, "Add address {} to {}", peer_addr, peer_id);
            Ok((inserted.clone(), CachedAddressActiveModel::new(inserted.id, peer_addr.clone()).insert(DATABASE_CONNECTIONS.get_cache_unchecked()).await?))
        },


    }
}