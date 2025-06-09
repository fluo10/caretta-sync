use sea_orm_migration::prelude::*;

pub mod m20220101_000001_create_cache_tables;

pub struct CacheMigrator;

#[async_trait::async_trait]
impl MigratorTrait for CacheMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_cache_tables::Migration)]
    }
}
