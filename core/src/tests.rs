use std::marker::PhantomData;
use std::{fs::create_dir_all, path::PathBuf, sync::LazyLock};
use crate::context::ServerContext;

use crate::config::{
    LogConfig, ParsedConfig, PartialLogConfig, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, RpcConfig, StorageConfig
};
use crate::models::migration::m20220101_000001_create_table;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tempfile::TempDir;
use tokio::sync::OnceCell;
use url::Url;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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
        storage: Some(
            PartialStorageConfig {
                data_dir: Some(data_dir),
                cache_dir: Some(cache_dir),
            }
        ),
        rpc: Some(PartialRpcConfig::default(TEST_APP_NAME)),
        log: Some(PartialLogConfig::default())
    }
});

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
