use caretta_id::CarettaId;
use chrono::{DateTime, Local};
use iroh::Endpoint;
use sea_orm::{
    ActiveValue::{self, Set},
    entity::prelude::*,
};

use crate::types::{Database, EndpointPublicKey, EndpointSecretKey};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "authorized_device")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub public_id: CarettaId,
    pub public_key: EndpointPublicKey,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Model {
    async fn insert(db: &Database, endpoint: &Endpoint) -> Result<Self, DbErr> {
        ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            public_id: ActiveValue::Set(CarettaId::now_unix()),
            public_key: ActiveValue::Set(endpoint.id().into()),
            name: ActiveValue::Set(gethostname::gethostname().to_string_lossy().to_string()),
            created_at: ActiveValue::Set(Local::now()),
            updated_at: ActiveValue::Set(Local::now()),
        }
        .insert(db.as_ref())
        .await
    }

    async fn from_db(db: &Database, id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(id).one(db.as_ref()).await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let db = crate::tests::database().await;
        let endpoint = crate::tests::iroh_endpoint().await;
        let model = Model::insert(db, endpoint).await.unwrap();
        assert_eq!(
            model,
            Model::from_db(db, model.id.clone()).await.unwrap().unwrap()
        );
    }
}
