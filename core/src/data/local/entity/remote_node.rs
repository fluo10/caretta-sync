//! Structs about cached remote_node.

use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use mtid::Dtid;
use rand::Rng;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use uuid::Uuid;

use crate::data::local::types::PublicKeyBlob;

/// RemoteNode information cached in local database.
///
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - Acutualy this is some sort of junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual remote_node information is managed by iroh endpoint and not contained in this model.
/// - Once a remote_node is authorized, it is assigned a global (=synced) ID as authorized_remote_node so essentially this local id targets unauthorized remote_nodes.
///
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "remote_node")]
pub struct Model {
    /// serial primary key.
    #[sea_orm(primary_key)]
    pub id: u32,

    /// public DITD of remote_node.
    /// this id is use only the node itself and not synced so another node has different local_remote_node_id even if its public_key is same.
    pub public_id: Dtid,

    /// Iroh public key
    pub public_key: PublicKeyBlob,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AuthorizationRequest,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AuthorizationRequest => {
                Entity::has_many(super::authorization_request::Entity).into()
            }
        }
    }
}

impl Related<super::authorization_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthorizationRequest.def()
    }
}

impl From<PublicKey> for ActiveModel {
    fn from(value: PublicKey) -> Self {
        let mut rng = rand::thread_rng();
        let dtid: Dtid = rng.r#gen();
        Self {
            public_key: Set(value.into()),
            public_id: Set(dtid),
            ..Default::default()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    #[cfg(test)]
    pub fn new_test() -> Self {
        use iroh::SecretKey;

        let mut rng = rand::thread_rng();
        let public_key = SecretKey::generate(rng).public();
        ActiveModel::from(public_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::local::migration::TestMigrator, tests::TEST_CONFIG};
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

        let active_model = ActiveModel::new_test();
        let model = active_model.clone().insert(db).await.unwrap();
        assert_eq!(model.public_id, active_model.public_id.unwrap());
        assert_eq!(model.public_key, active_model.public_key.unwrap());
    }
}
