use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "note_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub note_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: Uuid,
    #[sea_orm(belongs_to, from = "note_id", to = "id")]
    pub note: Option<super::note::Entity>,
    #[sea_orm(belongs_to, from = "note_id", to = "id")]
    pub tag: Option<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}