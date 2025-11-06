use std::{marker::PhantomData, sync::LazyLock};

use caretta_sync_core::{
    context::BackendContext,
    parsed_config::{ParsedConfig, ParsedLogConfig, ParsedRpcConfig, ParsedStorageConfig},
};
use tokio::sync::OnceCell;

use crate::models::migration::m20220101_000001_create_table;

const TEST_APP_NAME: &str = "caretta-sync-test";

pub static CONFIG: LazyLock<ParsedConfig> = LazyLock::new(|| {
    let test_dir = tempfile::Builder::new()
        .prefix(TEST_APP_NAME)
        .tempdir()
        .unwrap()
        .keep();
    let data_dir = test_dir.join("data");
    let cache_dir = test_dir.join("cache");

    ParsedConfig {
        p2p: None,
        storage: Some(ParsedStorageConfig {
            data_dir: Some(data_dir),
            cache_dir: Some(cache_dir),
        }),
        rpc: Some(ParsedRpcConfig::default(TEST_APP_NAME)),
        log: Some(ParsedLogConfig::default()),
    }
});

pub static SERVER_CONTEXT: OnceCell<BackendContext> = OnceCell::const_new();
pub async fn get_server_context() -> &'static BackendContext {
    SERVER_CONTEXT
        .get_or_init(|| async {
            BackendContext::new(
                "caretta_sync_test",
                (*CONFIG).clone(),
                PhantomData::<TestMigrator>,
            )
            .await
            .unwrap()
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
