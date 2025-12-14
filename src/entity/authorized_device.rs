use caretta_id::CarettaId;
use chrono::{DateTime, Local};
use iroh::Endpoint;
use sea_orm::{ActiveValue::{self, Set}, entity::prelude::*};

use crate::{types::{EndpointPublicKey, EndpointSecretKey}};


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
    async fn new<C>(ctx: &C) -> Result<Self, DbErr>
    where 
        C: AsRef<DatabaseConnection> + AsRef<Endpoint>
    {
        ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            public_id: ActiveValue::Set(CarettaId::now_unix()),
            public_key: ActiveValue::Set(<C as AsRef<Endpoint>>::as_ref(ctx).id().into()),
            name: ActiveValue::Set(gethostname::gethostname().to_string_lossy().to_string()),
            created_at: ActiveValue::Set(Local::now()),
            updated_at: ActiveValue::Set(Local::now()),
        }.insert(<C as AsRef<DatabaseConnection>>::as_ref(ctx)).await
    }

    async fn from_db<C>(ctx: &C, id: Uuid) -> Result<Option<Self>, DbErr> 
    where
        C: AsRef<DatabaseConnection>
    {
        Entity::find_by_id(id).one(ctx.as_ref()).await

    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let ctx  = crate::tests::context().await;
        let model = Model::new(ctx).await.unwrap();
        assert_eq!(model, Model::from_db(ctx, model.id.clone()).await.unwrap().unwrap());
    }
}