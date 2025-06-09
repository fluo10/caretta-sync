pub mod cache;
pub mod main;

use sea_orm_migration::{prelude::*, schema::*};

#[async_trait::async_trait]
pub trait TableMigration {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> ;
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>;
}