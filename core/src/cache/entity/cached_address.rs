use std::str::FromStr;

use chrono::{Days, Local};
use libp2p::{multiaddr, Multiaddr, PeerId};
use prost_types::Timestamp;
use sea_orm::{entity::{
    prelude::*, *
}, sea_query};
use serde::{Deserialize, Serialize};

use crate::{cache, data::value::{MultiaddrValue, PeerIdValue}, rpc::proto::CachedAddressMessage, utils::utc_to_timestamp};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cached_address")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub cached_peer_id: u32,
    #[sea_orm(indexed)]
    pub multiaddress: MultiaddrValue,
}


#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::CachedPeerEntity",
        from = "Column::CachedPeerId",
        to = "super::CachedPeerColumn::Id"
    )]
    CachedPeer,
}
impl Related<super::CachedPeerEntity> for Entity {
    fn to() -> RelationDef {
        Relation::CachedPeer.def()
    }
}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(cached_peer_id: u32, multiaddr: Multiaddr) -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            cached_peer_id: Set(cached_peer_id),
            multiaddress: Set(MultiaddrValue::from(multiaddr)),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..Default::default()
        }
    }
}

impl From<Model> for CachedAddressMessage {
    fn from(a: Model) -> Self {
        Self {
            number: a.id,
            created_at: Some(utc_to_timestamp(a.created_at)),
            updated_at: Some(utc_to_timestamp(a.updated_at)),
            multiaddress: Multiaddr::from(a.multiaddress).to_string(),
        }
    }
}