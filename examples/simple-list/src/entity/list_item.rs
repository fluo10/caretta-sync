use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "list_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub is_trashed: bool,
    #[sea_orm(indexed)]
    pub content: String,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new() -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            id: Set(Uuid::new_v4()),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            is_trashed: Set(false),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::global::GLOBAL;

     #[tokio::test]
    async fn check_insert_node() {
        let db = crate::tests::get_or_init_temporary_database().await;
        
        ActiveModel{
            content: Set("test note".to_owned()),
            ..ActiveModel::new()
        }.insert(db).await.unwrap();
    }

}