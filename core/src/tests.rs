use std::{fs::create_dir_all, path::PathBuf, sync::LazyLock};
use crate::context::ServerContext;
use crate::example::config::ExampleParsedConfig;
use crate::example::migrator::ExampleMigrator;

use crate::config::{
    ParsedConfig, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, RpcConfig, StorageConfig
};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tempfile::TempDir;
use tokio::sync::OnceCell;
use url::Url;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

const TEST_APP_NAME: &str = "caretta-sync-test";

pub static CONFIG: LazyLock<ExampleParsedConfig> = LazyLock::new(|| {
    let test_dir = tempfile::Builder::new()
        .prefix(TEST_APP_NAME)
        .tempdir()
        .unwrap()
        .keep();
    let data_dir = test_dir.join("data");
    let cache_dir = test_dir.join("cache");

    ExampleParsedConfig {
        p2p: None,
        storage: Some(
            PartialStorageConfig {
                data_dir: Some(data_dir),
                cache_dir: Some(cache_dir),
            }
        ),
        rpc: Some(PartialRpcConfig::default(TEST_APP_NAME)),
    }
});

pub static SERVER_CONTEXT: OnceCell<ServerContext> = OnceCell::const_new();
pub async fn get_server_context() -> &'static ServerContext {
    SERVER_CONTEXT.get_or_init(|| async  {
        ServerContext::from_parsed_config::<_,ExampleMigrator>((*CONFIG).clone()).await.unwrap()
    }).await
}