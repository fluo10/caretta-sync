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
    pub peer_id: String,
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
    use super::*;

    use libp2p::identity;
    use crate::global::GLOBAL;

     #[tokio::test]
    async fn check_insert_node() {
        let db = crate::global::get_or_init_temporary_main_database().await;
        
        ActiveModel{
            peer_id: Set(identity::Keypair::generate_ed25519().public().to_peer_id().to_string()),
            ..ActiveModel::new()
        }.insert(db).await.unwrap();
    }

}