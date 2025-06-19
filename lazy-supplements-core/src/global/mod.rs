use std::{any::type_name, collections::HashMap, net::{IpAddr, Ipv4Addr}, path::{Path, PathBuf}, sync::LazyLock};

use crate::{config::{P2pConfig, PartialP2pConfig, StorageConfig}, error::Error};
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId};
use sea_orm::{prelude::*, Database};
use sea_orm_migration::MigratorTrait;
use tokio::sync::{OnceCell, RwLock};

mod peers;
pub use peers::GlobalPeers;
mod config;
pub use config::STORAGE_CONFIG;
mod database_connection;
pub use database_connection::*;
use uuid::{ContextV7, Timestamp, Uuid};

pub fn generate_uuid() -> Uuid {
    Uuid::new_v7(Timestamp::now(ContextV7::new()))
}

pub static PRODUCT_NAME: LazyLock<String> = LazyLock::new(|| {
    env!("CARGO_PKG_NAME").to_string()
});

pub static DEFAULT_LISTEN_IPS: &[IpAddr] = &[IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))];

pub static DEFAULT_CONFIG_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".toml")
});


pub static DEFAULT_DATABASE_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".sqlite")
});
fn uninitialized_message<T>(var: T) -> String {
    format!("{} is uninitialized!", &stringify!(var))
}

struct SimpleGlobal<T> {
    inner: OnceCell<T>
}

impl<T> SimpleGlobal<T> { 
    pub const fn const_new() -> Self {
        Self{inner: OnceCell::const_new()}
    }
    pub async fn get_or_init(&'static self, source: T) -> &'static T {
        self.inner.get_or_init(|| async {
            source
        }).await
    }
    pub fn get(&'static self) -> Option<&'static T> {
        self.inner.get()
    }
    pub fn get_and_unwrap(&'static self) -> &'static T {
        self.get().expect(&format!("{} is uninitialized!", &stringify!(self)))
    }
}

struct GlobalRwLock<T> {
    inner: OnceCell<RwLock<T>>
}

impl<T> GlobalRwLock<T> {
    pub const fn const_new() -> Self {
        Self{inner: OnceCell::const_new()}
    }
    async fn write(&'static self) -> tokio::sync::RwLockWriteGuard<'_ ,T> {
        self.get_peers_once_cell().get().expect(UNINITIALIZED_MESSAGE).write().await
    }
    async fn read(&'static self) -> RwLockReadGuard<'_, T> {
        self.get_peers_once_cell().get().expect(UNINITIALIZED_MESSAGE).read().await

    }
}

#[cfg(test)]
pub struct TestGlobal {
    pub storage_config: &'static StorageConfig,
    pub data_database_connection: &'static DatabaseConnection,
    pub cache_database_connection: &'static DatabaseConnection,
}

#[cfg(test)]
mod tests {
    use crate::{cache::migration::CacheMigrator, data::migration::DataMigrator};

    use super::*;
    static TEST_DATA_DIRECTORY: LazyLock<PathBuf> = todo!();
    static TEST_DATA_DATABASE_PATH: LazyLock<PathBuf> = todo!();
    static TEST_CACHE_DIRECTORY: LazyLock<PathBuf> = todo!();
    static TEST_CACHE_DATABASE_PATH: LazyLock<PathBuf> = todo!();
    static TEST_STORAGE_CONFIG: LazyLock<StorageConfig> = todo!();
    
    pub async fn get_or_try_init_test() -> TestGlobal {
        TestGlobal {
            storage_config: get_or_init_storage_config(StorageConfig{data_directory: TEST_DATA_DIRECTORY.clone(), cache_directory: TEST_CACHE_DIRECTORY.clone()}).await,
            data_database_connection: get_or_try_init_data_database_connection(&*TEST_DATA_DATABASE_PATH, DataMigrator ).await.unwrap(),
            cache_database_connection: get_or_try_init_cache_database_connection(&*TEST_CACHE_DATABASE_PATH, CacheMigrator).await.unwrap(),
        }
    }
}
