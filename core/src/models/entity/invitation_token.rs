use core::time;
use std::convert::Infallible;

use chrono::{DateTime, Duration, Local};
use mtid::Dtid;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use uuid::Uuid;

use crate::{models::{types::TokenStatus, ModelsError}, token::InvitationToken};

/// Request of node authorization.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "invitation_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    /// Public [`Dtid`]
    pub public_id: Dtid,
    pub created_at: DateTime<Local>,
    pub expires_at: DateTime<Local>,
    pub closed_at: Option<DateTime<Local>>,
    pub status: TokenStatus
}

impl Model {
    pub async fn new(db: &DatabaseConnection, duration: Duration) -> Result<Self, ModelsError> {
        Ok(ActiveModel::new(duration).insert(db).await?)
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Entity {
    pub fn find_by_public_id(id: Dtid) -> Select<Entity>{
        Self::find().filter(Column::PublicId.eq(id))
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self,db: &C,insert:bool) ->  Result<Self,DbErr>
        where C:ConnectionTrait,
    {   
        if insert {
            for _ in 0..100 {
                let public_id = Dtid::random();
                if let Some(_) = Entity::find_by_public_id(public_id).one(db).await? {
                    continue;
                } else {
                    self.public_id = Set(public_id);
                    break;
                }
            }

        }
        Ok(self)
    }
}

impl ActiveModel {
    fn new(duration: Duration) -> Self {
        let timestamp = Local::now();
        Self {
            created_at: Set(timestamp.clone()),
            expires_at: Set(timestamp + duration),
            status: Set(TokenStatus::default()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{entity::invitation_token, migration::TestMigrator},
        tests::TEST_CONFIG,
    };
    use iroh::{PublicKey, SecretKey};
    use rand::Rng;
    use sea_orm::{sea_query::Token, ActiveValue::Set};

    #[tokio::test]
    async fn insert() {
        let db = crate::global::LOCAL_DATABASE_CONNECTION
            .get_or_try_init::<_, TestMigrator>(&TEST_CONFIG.storage.get_local_database_path())
            .await
            .unwrap();

        let active_model = ActiveModel {
            created_at: Set(chrono::Local::now()),
            expires_at: Set(chrono::Local::now()),
            status: Set(TokenStatus::default()),
            ..Default::default()
        };
        let model = active_model.clone().insert(db).await.unwrap();
        assert_eq!(active_model.expires_at.unwrap(), model.expires_at);
    }
}
