use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
use lazy_supplements_core::data::syncable::*;
use lazy_supplements_macros::SyncableModel;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SyncableModel)]
#[sea_orm(table_name = "syncable")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[syncable(id)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    #[syncable(timestamp)]
    pub created_at: DateTimeUtc,
    pub table_name: String,

    #[syncable(author_id)]
    pub updated_by: Uuid,
    pub record_id: Uuid,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation{}
    
impl ActiveModelBehavior for ActiveModel {}
