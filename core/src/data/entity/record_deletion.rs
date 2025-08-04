use chrono::Local;
use sea_orm::{entity::{
    prelude::*, *
}, sea_query::table};
use serde::{Deserialize, Serialize};
use crate::data::syncable::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[cfg_attr(feature="macros", derive(SyncableModel))]
#[sea_orm(table_name = "record_deletion")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[cfg_attr(feature="macros", syncable(id))]
    pub id: Uuid,
    #[sea_orm(indexed)]
    #[cfg_attr(feature="macros", syncable(timestamp))]
    pub created_at: DateTimeUtc,
    #[cfg_attr(feature="macros", syncable(author_id))]
    pub created_by: Uuid,
    pub table_name: String,
    pub record_id: Uuid,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation{}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(node_id: Uuid, table_name: String, record_id: Uuid) -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            id: Set(crate::global::generate_uuid()),
            created_at: Set(timestamp),
            created_by: Set(node_id),
            table_name: Set(table_name),
            record_id: Set(record_id),
        }
    }
}

