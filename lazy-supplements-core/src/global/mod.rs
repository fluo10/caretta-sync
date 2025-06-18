use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}, path::{Path, PathBuf}, sync::LazyLock};

use crate::{config::{P2pConfig, PartialP2pConfig, StorageConfig}, error::Error};
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId};
use sea_orm::{prelude::*, Database};
use sea_orm_migration::MigratorTrait;
use tokio::sync::{OnceCell, RwLock};

mod peers;
pub use peers::GlobalPeers;
mod storage_config;
mod database_connection;
pub use database_connection::GlobalDatabaseConnection;
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

#[cfg(any(test, feature="test"))]
pub struct TestGlobal {
    p2p_config: OnceCell<P2pConfig>,
    storage_config: OnceCell<StorageConfig>,
    data_database_connection: OnceCell<DatabaseConnection>,
    cache_database_connection: OnceCell<DatabaseConnection>,
}
#[cfg(any(test, feature="test"))]
pub static GLOBAL: TestGlobal = TestGlobal{
    p2p_config: OnceCell::const_new(),
    storage_config: OnceCell::const_new(),
    data_database_connection: OnceCell::const_new(),
    cache_database_connection: OnceCell::const_new(),
};
