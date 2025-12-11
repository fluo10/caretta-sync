use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(has_many)]
    pub attachement: HasMany<super::attachement::Entity>,
    #[sea_orm(has_many)]
    pub note: HasMany<super::note::Entity>,
    #[sea_orm(has_many)]
    pub thread: HasMany<super::thread::Entity>,
    #[sea_orm(has_many)]
    pub tag: HasMany<super::tag::Entity>,

}

impl ActiveModelBehavior for ActiveModel {}