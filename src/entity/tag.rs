use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(has_one)]
    pub workspace: HasOne<super::workspace::Entity>,
    #[sea_orm(has_many, via = "tag_thread")]
    pub threads: HasMany<super::thread::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}