use sea_orm_migration::prelude::*;

pub mod m20220101_000001_create_main_tables;

#[cfg(any(test, feature="test"))]
pub struct MainMigrator;

#[cfg(any(test, feature="test"))]
#[async_trait::async_trait]
impl MigratorTrait for MainMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_main_tables::Migration)]
    }
}
