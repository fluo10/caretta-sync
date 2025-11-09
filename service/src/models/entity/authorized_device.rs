//! Structs about cached remote_node.

use iroh::PublicKey;
use mtid::Dtid;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::models::types::PublicKeyBlob;

/// RemoteNode information cached in local database.
///
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - Acutualy this is some sort of junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual remote_node information is managed by iroh endpoint and not contained in this model.
/// - Once a remote_node is authorized, it is assigned a global (=synced) ID as authorized_remote_node so essentially this local id targets unauthorized remote_nodes.
///
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "authorized_device")]
pub struct Model {
    /// serial primary key.
    #[sea_orm(primary_key)]
    pub id: u32,

    pub uuid: Uuid,

    /// public [`Dtid`] of the node.
    pub public_id: Dtid,

    /// Iroh public key
    pub public_key: PublicKeyBlob,

    /// Name of the node.
    pub name: String,

    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use iroh::SecretKey;
    use rand::Rng;
    use sea_orm::ActiveValue::Set;

    use super::*;
    #[tokio::test]
    async fn insert() {
        let db: &DatabaseConnection = crate::tests::backend_conext().await.as_ref();

        let active_model = ActiveModel {
            uuid: Set(Uuid::now_v7()),
            public_id: Set(Dtid::random()),
            public_key: Set(PublicKeyBlob::from(
                iroh::SecretKey::generate(&mut rand::rng()).public(),
            )),
            name: Set(String::from("test")),
            created_at: Set(Local::now()),
            updated_at: Set(Local::now()),
            ..Default::default()
        };
        let model = active_model.clone().insert(db).await.unwrap();
        assert_eq!(model.public_id, active_model.public_id.unwrap());
        assert_eq!(model.public_key, active_model.public_key.unwrap());
    }
}
