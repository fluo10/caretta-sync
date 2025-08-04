use chrono::Local;
use libp2p::PeerId;
use sea_orm::entity::{
    *,
    prelude::*
};
use serde::{Deserialize, Serialize};

use crate::data::value::PeerIdValue;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "trusted_node")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub synced_at: Option<DateTimeUtc>,
    #[sea_orm(indexed)]
    pub peer_id: PeerIdValue,
    #[sea_orm(column_type = "Text")]
    pub note: String,
    pub is_prefered: bool,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(peer_id: PeerId, note: String) -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            id: Set(crate::global::generate_uuid()),
            peer_id: Set(PeerIdValue::from(peer_id)),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            synced_at: Set(None),
            is_prefered: Set(false),
            note: Set(note),
        }
    }
}

