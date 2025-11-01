use crate::models::migration::m20220101_000001_create_table;

pub struct ExampleMigrator;

#[async_trait::async_trait]
impl sea_orm_migration::MigratorTrait for ExampleMigrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
