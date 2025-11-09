use caretta_sync_core::config::P2pConfig;
use iroh::{PublicKey, SecretKey};
use mtid::Dtid;
use sea_orm::{ActiveValue::Set, entity::prelude::*, sea_query::Mode};

use crate::model::types::SecretKeyBlob;

const ID: u32 = 0;
/// config saved in local database.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "p2p_config")]
pub struct Model {
    /// serial primary key.
    #[sea_orm(primary_key)]
    pub id: u32,
    pub enabled: bool,
    pub secret_key: SecretKeyBlob,
    pub enable_n0: bool,
    pub enable_mdns: bool,
}

impl Model {
    pub async fn get_or_try_init(db: &DatabaseConnection) -> Result<Self, DbErr> {
        if let Some(x) = Entity::find_by_id(ID).one(db).await? {
            Ok(x)
        } else {
            Ok(ActiveModel {
                id: Set(ID),
                enabled: Set(true),
                secret_key: Set(SecretKey::generate(&mut rand::rng()).into()),
                enable_n0: Set(true),
                enable_mdns: Set(true),
            }
            .insert(db)
            .await?)
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for P2pConfig {
    fn from(value: Model) -> Self {
        Self {
            enabled: value.enabled,
            secret_key: value.secret_key.into(),
            enable_mdns: value.enable_mdns,
            enable_n0: value.enable_n0,
        }
    }
}

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

        let model = Model::get_or_try_init(db).await.unwrap();
        assert_eq!(model.id, ID);
        let model2 = Model::get_or_try_init(db).await.unwrap();
        assert_eq!(model, model2);
    }
}
