
use chrono::{DateTime, Local};
use mtid::Dtid;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::models::types::{NodeVerificationStatus, PublicKeyBlob};

/// Request of node authorization.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "node_verification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub uuid: Uuid,
    pub public_id: Dtid,
    pub public_key: PublicKeyBlob,
    pub status: NodeVerificationStatus,
    pub node_info: String,
    pub created_at: DateTime<Local>,
    pub closed_at: Option<DateTime<Local>>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
             entity::node_verification, migration::TestMigrator,
        },
        tests::TEST_CONFIG,
    };
    use iroh::{PublicKey, SecretKey};
    use rand::Rng;
    use sea_orm::ActiveValue::Set;

    #[tokio::test]
    async fn insert() {
        let db = crate::global::LOCAL_DATABASE_CONNECTION
            .get_or_try_init::<_, TestMigrator>(&TEST_CONFIG.storage.get_local_database_path())
            .await
            .unwrap();

        let active_model = ActiveModel {
            uuid: Set(uuid::Uuid::now_v7()),
            public_id: Set(Dtid::random()),
            public_key: Set(iroh::SecretKey::generate(&mut rand::rng()).public().into()),
            status: Set(NodeVerificationStatus::Pending),
            node_info: Set(String::from("test_node")),
            created_at: Set(chrono::Local::now()),
            ..Default::default()
        };
        let model = active_model
            .clone()
            .insert(db)
            .await
            .unwrap();
        assert_eq!(
            active_model.uuid.unwrap(),
            model.uuid
        );
        assert_eq!(
            PublicKey::try_from(active_model.public_key.unwrap()).unwrap(),
            PublicKey::try_from(model.public_key).unwrap()
        )
    }
}
