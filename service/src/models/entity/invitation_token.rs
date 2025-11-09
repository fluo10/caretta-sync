use core::time;
use std::convert::Infallible;

use chrono::{DateTime, Duration, DurationRound, Local, SubsecRound};
use mtid::Dtid;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use uuid::Uuid;

use crate::{
    invitation_token::InvitationToken,
    models::{ModelsError, types::TokenStatus},
};

/// Request of node authorization.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "invitation_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    /// Public [`Dtid`]
    pub public_id: Dtid,
    pub created_at: DateTime<Local>,

    /// The timestamp when the token expires
    ///
    /// Since the token does not contain subseconds, this timestamp is also rounded to the newarest second.
    pub expires_at: DateTime<Local>,
    pub closed_at: Option<DateTime<Local>>,
    pub status: TokenStatus,
}

impl Model {
    pub async fn new(db: &DatabaseConnection, duration: Duration) -> Result<Self, ModelsError> {
        Ok(ActiveModel::new(duration).insert(db).await?)
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Entity {
    pub fn find_by_public_id(id: Dtid) -> Select<Entity> {
        Self::find().filter(Column::PublicId.eq(id))
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
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
            expires_at: Set((timestamp + duration).round_subsecs(0)),
            status: Set(TokenStatus::default()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iroh::{PublicKey, SecretKey};
    use rand::Rng;
    use sea_orm::{ActiveValue::Set, sea_query::Token};

    #[tokio::test]
    async fn insert() {
        let db: &DatabaseConnection = crate::tests::backend_conext().await.as_ref();

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
