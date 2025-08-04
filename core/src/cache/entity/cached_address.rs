use std::str::FromStr;

use chrono::{Days, Local};
use libp2p::{multiaddr, Multiaddr, PeerId};
use sea_orm::{entity::{
    prelude::*, *
}, sea_query};
use serde::{Deserialize, Serialize};

use crate::{cache, data::value::{MultiaddrValue, PeerIdValue}};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cached_address")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub last_used_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub cached_peer_id: u32,
    #[sea_orm(indexed)]
    pub address: MultiaddrValue,
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
            address: Set(MultiaddrValue::from(multiaddr)),
            created_at: Set(timestamp),
            last_used_at: Set(timestamp),
            ..Default::default()
        }
    }
}
