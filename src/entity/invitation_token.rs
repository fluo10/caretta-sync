use caretta_id::CarettaId;
use chrono::Local;
use rand::Rng;
use sea_orm::{ActiveValue, Database, entity::prelude::* };

use crate::{types::TokenStatus};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "invitation_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: CarettaId,
    pub secret: i64,
    pub created_at: chrono::DateTime<Local>,
    pub closed_at: Option<chrono::DateTime<Local>>,
    pub status: TokenStatus,
}

impl Model {
    async fn new<C>(ctx: &C) -> Result<Self, DbErr> 
    where
        C: AsRef<DatabaseConnection>
    {
        ActiveModel {
            secret : ActiveValue::Set(rand::rng().random()),
            created_at : ActiveValue::Set(Local::now()),
            closed_at : ActiveValue::Set(None),
            status: ActiveValue::Set(TokenStatus::Pending),
            ..Default::default()
        }.insert(ctx.as_ref()).await
    }
    async fn from_db<C>(ctx: &C, id: CarettaId) -> Result<Option<Self>, DbErr>
    where
        C: AsRef<DatabaseConnection>
    {
        Entity::find_by_id(id).one(ctx.as_ref()).await
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let ctx = crate::tests::context().await;
        let model = Model::new(ctx).await.unwrap();
        assert_eq!(model, Model::from_db(ctx, model.id.clone()).await.unwrap().unwrap());
    }
}