use std::{marker::PhantomData, sync::LazyLock};

use caretta_sync_core::{config::StorageConfig, context::ServiceContext};
use tokio::sync::OnceCell;

use crate::model::migration::m20220101_000001_create_table;

const TEST_APP_NAME: &str = "caretta-sync-test";

pub static BACKEND_CONTEXT: OnceCell<ServiceContext> = OnceCell::const_new();
pub async fn backend_conext() -> &'static ServiceContext {
    BACKEND_CONTEXT
        .get_or_init(|| async move {
            let test_dir = tempfile::Builder::new()
                .prefix(TEST_APP_NAME)
                .tempdir()
                .unwrap()
                .keep();
            let data_dir = test_dir.join("data");
            let cache_dir = test_dir.join("cache");
            let storage_config = StorageConfig {
                data_dir,
                cache_dir,
            };
            let database_connection = storage_config
                .to_database_connection(PhantomData::<TestMigrator>)
                .await;
            let iroh_router = None;
            ServiceContext {
                app_name: TEST_APP_NAME,
                storage_config,
                database_connection,
                iroh_router,
            }
        })
        .await
}

pub struct TestMigrator;

#[async_trait::async_trait]
impl sea_orm_migration::MigratorTrait for TestMigrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
