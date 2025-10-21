use std::os::unix::raw::time_t;

use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use mtid::Dtid;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Request of node authorization.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "authorization_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub uuid: Uuid,
    pub public_id: Dtid,
    pub remote_node_id: u32,
    pub created_at: DateTime<Local>,
    pub closed_at: Option<DateTime<Local>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RemoteNode,
    ReceivedAuthorizationRequest,
    SentAuthorizationRequest,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RemoteNode => Entity::belongs_to(super::remote_node::Entity)
                .from(Column::RemoteNodeId)
                .to(super::remote_node::Column::Id)
                .into(),
            Self::ReceivedAuthorizationRequest => {
                Entity::has_one(super::received_authorization_request::Entity).into()
            }
            Self::SentAuthorizationRequest => {
                Entity::has_one(super::sent_authorization_request::Entity).into()
            }
        }
    }
}

impl Related<super::remote_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RemoteNode.def()
    }
}

impl Related<super::received_authorization_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceivedAuthorizationRequest.def()
    }
}

impl Related<super::sent_authorization_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SentAuthorizationRequest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    #[cfg(test)]
    pub fn new_test(remote_node: &super::remote_node::Model) -> Self {

        use sea_orm::ActiveValue::Set;

        Self {
            uuid: Set(uuid::Uuid::now_v7()),
            public_id: Set(Dtid::random()),
            remote_node_id: Set(remote_node.id),
            created_at: Set(chrono::Local::now()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::local::{
            RemoteNodeActiveModel, entity::authorization_request, migration::TestMigrator,
        },
        tests::TEST_CONFIG,
    };
    use iroh::SecretKey;
    use rand::Rng;
    use sea_orm::ActiveValue::Set;

    use super::*;
    #[tokio::test]
    async fn insert() {
        let db = crate::global::LOCAL_DATABASE_CONNECTION
            .get_or_try_init(&TEST_CONFIG.storage.get_local_database_path(), TestMigrator)
            .await
            .unwrap();
        let mut rng = rand::thread_rng();

        let remote_node_public_key = SecretKey::generate(&mut rng).public();
        let remote_node_active_model = RemoteNodeActiveModel::from(remote_node_public_key);
        let remote_node_model = remote_node_active_model.clone().insert(db).await.unwrap();

        let authorization_request_active_model = ActiveModel {
            uuid: Set(uuid::Uuid::now_v7()),
            public_id: Set(Dtid::random()),
            remote_node_id: Set(remote_node_model.id),
            created_at: Set(chrono::Local::now()),
            ..Default::default()
        };
        let authorization_request_model = authorization_request_active_model
            .clone()
            .insert(db)
            .await
            .unwrap();
        assert_eq!(
            authorization_request_active_model.uuid.unwrap(),
            authorization_request_model.uuid
        );
    }
}
