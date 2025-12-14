use std::{marker::PhantomData, path::PathBuf, sync::{Arc, LazyLock}};

use crate::{config::{P2pConfig, StorageConfig}, entity::device_config};
use caretta_sync_migration::Migrator;
use iroh::Endpoint;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

static STORAGE_CONFIG: OnceCell<StorageConfig> = OnceCell::const_new();
async fn storage_config() -> &'static StorageConfig {
    STORAGE_CONFIG.get_or_init(|| async move {
        let dir = tempfile::Builder::new()
            .prefix("caretta_brain_test")
            .tempdir()
            .unwrap()
            .keep();
        let data_dir = dir.join("data");
        let cache_dir = dir.join("cache");
        StorageConfig { data_dir, cache_dir }
    }).await
}
static P2P_CONFIG: OnceCell<P2pConfig> = OnceCell::const_new();
async fn p2p_config() -> &'static P2pConfig {
    P2P_CONFIG.get_or_init(|| async move {
        P2pConfig::from(device_config().await.clone())
    }).await
}

static DATABASE_CONNECTION: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

async fn database_connection() -> &'static Arc<DatabaseConnection> {
    DATABASE_CONNECTION.get_or_init(|| async move {
        Arc::new(storage_config().await.to_database_connection().await)
    }).await
}

static DEVICE_CONFIG: OnceCell<device_config::Model> = OnceCell::const_new();

async fn device_config() -> &'static device_config::Model {
    DEVICE_CONFIG.get_or_init(|| async move {
        device_config::Model::get_or_try_init(database_connection().await).await.unwrap()
    }).await
}
pub struct TestContext{
    pub database_connection: Arc<DatabaseConnection>,
    pub iroh_endpoint: Endpoint
}

impl AsRef<DatabaseConnection> for TestContext {
    fn as_ref(&self) -> &DatabaseConnection {
        self.database_connection.as_ref()
    }
}

impl AsRef<Endpoint> for TestContext {
    fn as_ref(&self) -> &iroh::Endpoint {
        &self.iroh_endpoint
    }
}

static TEST_CONTEXT: OnceCell<TestContext> = OnceCell::const_new();
pub async fn context() -> &'static TestContext {
    TEST_CONTEXT.get_or_init(|| async move {
        TestContext {
            database_connection: database_connection().await.clone(),
            iroh_endpoint: p2p_config().await.spawn_iroh_endpoint().await.unwrap()
        }
    }).await
}