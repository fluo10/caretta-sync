
pub static SERVER_CONTEXT: OnceCell<ServerContext> = OnceCell::const_new();
pub async fn get_server_context() -> &'static ServerContext {
    SERVER_CONTEXT.get_or_init(|| async  {
        ServerContext::new("caretta_sync_test", (*CONFIG).clone(), PhantomData::<TestMigrator>).await.unwrap()
    }).await
}

pub struct TestMigrator;

#[async_trait::async_trait]
impl sea_orm_migration::MigratorTrait for TestMigrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
