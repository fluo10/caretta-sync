use std::{marker::PhantomData, path::PathBuf, sync::{Arc, LazyLock}};

use crate::{config::{P2pConfig, StorageConfig}, entity::device_config, types::Database};
use caretta_sync_migration::Migrator;
use iroh::Endpoint;
use sea_orm::{ DatabaseConnection};
use tokio::sync::OnceCell;

pub static APP_NAME: &str = "caretta-sync-test";

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

static DATABASE: OnceCell<Database> = OnceCell::const_new();

pub async fn database() -> &'static Database {
    DATABASE.get_or_init(|| async move {
        storage_config().await.open_database().await
    }).await
}

static DEVICE_CONFIG: OnceCell<device_config::Model> = OnceCell::const_new();

async fn device_config() -> &'static device_config::Model {
    DEVICE_CONFIG.get_or_init(|| async move {
        device_config::Model::get_or_try_init(database().await).await.unwrap()
    }).await
}

static IROH_ENDPOINT: OnceCell<Endpoint> = OnceCell::const_new();

pub async fn iroh_endpoint() -> &'static Endpoint {
    IROH_ENDPOINT.get_or_init(|| async move {
        p2p_config().await.spawn_iroh_endpoint(APP_NAME).await.unwrap()
    }).await
}