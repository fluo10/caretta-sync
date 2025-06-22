use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub synced_at: Option<DateTimeUtc>,
    #[sea_orm(indexed)]
    pub peer_id: String,
    #[sea_orm(column_type = "Text")]
    pub note: String,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new() -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::global::get_or_init_test_cache_database;

    use super::*;

    use libp2p::identity;

     #[tokio::test]
    async fn check_insert_node() {
        let db = get_or_init_test_cache_database().await;
        
        ActiveModel{
            peer_id: Set(identity::Keypair::generate_ed25519().public().to_peer_id().to_string()),
            note: Set("test note".to_owned()),
            ..ActiveModel::new()
        }.insert(db).await.unwrap();
    }

}