use sea_orm_migration::{prelude::*, schema::*};
use lazy_supplements_migration::m20220101_000001_create_lazy_supplements_tables;
mod m20220101_000002_create_simple_list_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000002_create_simple_list_tables::Migration),
            Box::new(m20220101_000001_create_lazy_supplements_tables::Migration)
        ]
    }
}
