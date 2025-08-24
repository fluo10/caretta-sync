use chrono::Local;
use sea_orm::{
    prelude::*,
    entity::{
        *,
        prelude::*
    }
};
use caretta_sync_core::data::syncable::*;
use caretta_sync_macros::SyncableModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SyncableModel)]
#[sea_orm(table_name = "syncable")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[syncable(id)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    #[syncable(timestamp)]
    pub created_at: DateTimeUtc,
    #[syncable(author_id)]
    pub created_by: Uuid,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation{}
    
impl ActiveModelBehavior for ActiveModel {}

#[test]
fn test_columns() {
    assert!(Column::Id.is_id());
    assert!(Column::CreatedAt.is_timestamp());
    assert!(Column::CreatedBy.is_author_id());
}