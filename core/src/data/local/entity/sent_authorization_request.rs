use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use mtid::Dtid;
use rand::Rng;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::data::local::entity::authorization_request;

/// Sent request of node authentication.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sent_authorization_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub authorization_request_id: u32,
    pub passcode: Dtid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AuthorizationRequest,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AuthorizationRequest => Entity::belongs_to(super::authorization_request::Entity)
                .from(Column::AuthorizationRequestId)
                .to(super::authorization_request::Column::Id)
                .into(),
        }
    }
}

impl Related<super::authorization_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthorizationRequest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    fn new(authorization_request: &super::authorization_request::Model) -> Self {
        Self {
            authorization_request_id: Set(authorization_request.id),
            passcode: Set(Dtid::random()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::local::{
            RemoteNodeActiveModel,
            entity::{authorization_request, sent_authorization_request},
            migration::TestMigrator,
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

        let remote_node_model = super::super::remote_node::ActiveModel::new_test()
            .insert(db)
            .await
            .unwrap();
        let authorization_request_model =
            super::super::authorization_request::ActiveModel::new_test(&remote_node_model)
                .insert(db)
                .await
                .unwrap();
        let sent_authorization_request_active_model =
            ActiveModel::new(&authorization_request_model);
        let sent_authorization_request_model = sent_authorization_request_active_model
            .clone()
            .insert(db)
            .await
            .unwrap();

        assert_eq!(
            sent_authorization_request_active_model.passcode.unwrap(),
            sent_authorization_request_model.passcode
        );
    }
}
