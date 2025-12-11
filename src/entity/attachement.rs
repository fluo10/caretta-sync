use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "attachement")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub workspace_id: Uuid,
    #[sea_orm(belongs_to, from = "workspace_id", to = "id")]
    pub workspace: HasOne<super::workspace::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}