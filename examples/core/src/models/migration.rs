#[derive(Debug)]
pub struct Migrator;

#[sea_orm_migration::async_trait::async_trait]
impl sea_orm_migration::MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(
            caretta_sync::service::models::migration::m20220101_000001_create_table::Migration,
        )]
    }
}
