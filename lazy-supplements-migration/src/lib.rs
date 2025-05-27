use sea_orm_migration::{prelude::*, schema::*};

pub mod m20220101_000001_create_lazy_supplements_tables;

pub struct Migrator;

#[async_trait::async_trait]
pub trait TableMigration {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> ;
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>;
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_lazy_supplements_tables::Migration)]
    }
}
