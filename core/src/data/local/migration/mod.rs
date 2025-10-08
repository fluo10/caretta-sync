use sea_orm_migration::*;

mod m20220101_000001_create_table;

#[cfg(test)]
pub struct TestMigrator;

#[cfg(test)]
#[async_trait::async_trait]
impl MigratorTrait for TestMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
        ]
    }
} 