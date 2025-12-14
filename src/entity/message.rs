use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sender: Uuid,
    pub body: String,
    pub thread_id: Uuid,
    #[sea_orm(belongs_to, from = "thread_id", to = "id")]
    pub thread: HasOne<super::thread::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}