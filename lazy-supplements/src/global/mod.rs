use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}, path::PathBuf, sync::LazyLock};

use crate::config::{NodeConfig, RawNodeConfig};
use libp2p::{Multiaddr, PeerId};
use sea_orm::DatabaseConnection;
use tokio::sync::{OnceCell, RwLock};

mod database;

pub static PRODUCT_NAME: LazyLock<String> = LazyLock::new(|| {
    env!("CARGO_PKG_NAME").to_string()
});

pub static DEFAULT_LISTEN_IPS: &[IpAddr] = &[IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))];

pub static DEFAULT_PORT: u16 = 8080;

pub static DEFAULT_CONFIG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::config_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});

pub static DEFAULT_CONFIG_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".toml")
});

pub static DEFAULT_CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_CONFIG_DIR_PATH.join(&*DEFAULT_CONFIG_FILE_NAME)
});

pub static DEFAULT_DATA_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::data_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});

pub static DEFAULT_DATABASE_FILE_NAME: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(String::new() + env!("CARGO_PKG_NAME") + ".sqlite")
});

pub static DEFAULT_DATABASE_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_DATA_DIR_PATH.join(&*DEFAULT_DATABASE_FILE_NAME)
});

pub static GLOBAL: Global = Global{
    node_config: OnceCell::const_new(),
    database: OnceCell::const_new(),
    peers: OnceCell::const_new(),
};
pub struct Global {
    pub node_config: OnceCell<NodeConfig>,
    pub database: OnceCell<DatabaseConnection>,
    pub peers: OnceCell<RwLock<HashMap<PeerId, Multiaddr>>>
}

#[cfg(test)]
pub use database::tests::get_or_init_temporary_database;

impl Global {
    pub fn get_node_config(&self) -> Option<&NodeConfig> {
        self.node_config.get()
    }
    pub async fn get_or_try_init_node_config(&self, config: NodeConfig) -> &NodeConfig {
        self.node_config.get_or_init(|| async {config}).await
    }
    pub async fn get_or_init_peers(&self) -> &RwLock<HashMap<PeerId, Multiaddr>> {
        self.peers.get_or_init(|| async {
            RwLock::new(HashMap::new())
        }).await
    }
    pub async fn read_peers(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<PeerId, Multiaddr>>{
        self.get_or_init_peers().await.read().await
    }
    pub async fn write_peers(&self) -> tokio::sync::RwLockWriteGuard<'_, HashMap<PeerId, Multiaddr>>{
        self.get_or_init_peers().await.write().await
    }
}

pub static DEFAULT_RAW_NODE_CONFIG: LazyLock<RawNodeConfig> = LazyLock::new(|| {
    RawNodeConfig {
        secret: None,
        database_path: Some(DEFAULT_DATABASE_FILE_PATH.to_path_buf()),
        listen_ips: Some(DEFAULT_LISTEN_IPS.to_vec()),
        port: Some(DEFAULT_PORT),
    }
});