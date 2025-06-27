use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
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
    pub fn new() -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            id: Set(crate::global::generate_uuid()),
            created_at: Set(timestamp),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::global::get_or_init_test_data_database;

    use super::*;

    use uuid::{Timestamp, Uuid};

     #[tokio::test]
    async fn check_insert_record_deletion() {
        let db = get_or_init_test_data_database().await;
        
        assert!(ActiveModel{
            table_name: Set("test_table".to_string()),
            record_id: Set(crate::global::generate_uuid()),
            ..ActiveModel::new()
        }.insert(db).await.is_ok());
    }

}