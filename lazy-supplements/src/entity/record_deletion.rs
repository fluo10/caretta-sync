use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "record_deletion")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
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
            id: Set(Uuid::new_v4()),
            created_at: Set(timestamp),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use uuid::Uuid;
    use crate::global::GLOBAL;

     #[tokio::test]
    async fn check_insert_record_deletion() {
        let db = GLOBAL.get_or_init_temporary_database().await;
        
        assert!(ActiveModel{
            table_name: Set("test_table".to_string()),
            record_id: Set(Uuid::new_v4()),
            ..ActiveModel::new()
        }.insert(db).await.is_ok());
    }

}