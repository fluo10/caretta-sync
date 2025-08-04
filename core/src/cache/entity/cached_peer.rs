use std::str::FromStr;

use chrono::{Days, Local};
use libp2p::{multiaddr, Multiaddr, PeerId};
use sea_orm::{entity::{
    prelude::*, *
}, sea_query};
use serde::{Deserialize, Serialize};

use crate::data::value::{MultiaddrValue, PeerIdValue};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cached_peer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub peer_id: PeerIdValue,
}


#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::CachedAddressEntity")]
    CachedAddress,
}

impl Related<super::CachedAddressEntity> for Entity {
    fn to() -> RelationDef {
        Relation::CachedAddress.def()
    }
}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(peer_id: PeerId) -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            peer_id: Set(PeerIdValue::from(peer_id)),
            created_at: Set(timestamp),
            ..Default::default()
        }
    }
}