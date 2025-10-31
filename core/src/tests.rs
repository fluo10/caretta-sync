use std::{fs::create_dir_all, path::PathBuf, sync::LazyLock};
use crate::example::migrator::ExampleMigrator;

use crate::config::{
    Config, PartialConfig, PartialP2pConfig, PartialRpcConfig, RpcConfig, StorageConfig,
};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tempfile::TempDir;
use tokio::sync::OnceCell;
use url::Url;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub static TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let test_dir = tempfile::Builder::new()
        .prefix("caretta-sync")
        .tempdir()
        .unwrap()
        .keep();
    let data_dir = test_dir.join("data");
    let cache_dir = test_dir.join("cache");

    Config {
        p2p: PartialP2pConfig::default()
            .with_new_secret_key()
            .try_into()
            .unwrap(),
        storage: StorageConfig {
            data_directory: data_dir,
            cache_directory: cache_dir,
        },
        rpc: RpcConfig {
            endpoint_url: Url::parse(
                &(String::from("unix://") + test_dir.join("socket.sock").to_str().unwrap()),
            )
            .unwrap(),
        },
    }
});

static TEST_DB: tokio::sync::OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_test_db() -> &'static DatabaseConnection {
    TEST_DB.get_or_init(|| async{
        let path = TEST_CONFIG.storage.get_local_database_path();
        let parent = path
            .parent()
            .expect("Database path should have parent directory");
        create_dir_all(parent).expect("Failed to create parent directory of database");
        let url = "sqlite://".to_owned()
            + path.to_str().expect("Invalid path string")
            + "?mode=rwc";
        let db = Database::connect(url).await.unwrap();
        ExampleMigrator::up(&db, None).await.unwrap();
        db
    }).await
}