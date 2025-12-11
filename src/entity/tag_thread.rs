use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "tag_thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub thread_id: Uuid,
    #[sea_orm(belongs_to, from = "thread_id", to = "id")]
    pub thread: HasOne<super::thread::Entity>,
    #[sea_orm(belongs_to, from = "tag_id", to = "id")]
    pub tag: HasOne<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}