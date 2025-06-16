use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}, path::{Path, PathBuf}, sync::LazyLock};

use crate::{config::{NodeConfig, RawNodeConfig}, error::Error};
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId};
use sea_orm::{prelude::*, Database};
use sea_orm_migration::MigratorTrait;
use tokio::sync::{OnceCell, RwLock};

mod database;
use database::GlobalDatabase;
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



pub static GLOBAL: Global = Global{
    node_config: OnceCell::const_new(),
    main_database: OnceCell::const_new(),
    cache_database: OnceCell::const_new(),
    peers: OnceCell::const_new(),
    
};
pub struct Global {
    pub node_config: OnceCell<NodeConfig>,
    pub main_database: OnceCell<DatabaseConnection>,
    pub cache_database: OnceCell<DatabaseConnection>,
    pub peers: OnceCell<RwLock<HashMap<PeerId, Multiaddr>>>,
}

impl Global {
    pub fn get_node_config(&self) -> Option<&NodeConfig> {
        self.node_config.get()
    }
    pub async fn get_or_init_node_config(&self, config: NodeConfig) -> &NodeConfig {
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
    pub async fn launch_swarm(&self) -> Result<(), Error> {
        let mut swarm = self.get_node_config().unwrap().clone().try_into_swarm().await?;
        loop{
            let swarm_event = swarm.select_next_some().await;
            tokio::spawn(async move{
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => {
                        println!("{event:?}");
                        event.run().await;
                    },
                    _ => {}
                }
            });
        }
    }
}

impl GlobalDatabase for Global {
    fn get_main_database(&self) -> Option<&DatabaseConnection> {
        self.main_database.get()
    }
    async fn get_or_try_init_main_database<T, U>(&self, path: T, _: U) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait,
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";

        Ok(self.main_database.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }
    fn get_cache_database(&self) -> Option<&DatabaseConnection> {
        self.cache_database.get()
    }
    async fn get_or_try_init_cache_database<T, U>(&self, path: T, _: U) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait,
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";

        Ok(self.cache_database.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }

}


#[cfg(test)]
pub use tests::{get_or_init_temporary_main_database, get_or_init_temporary_cache_database};
#[cfg(test)]
pub mod tests {
    use std::sync::LazyLock;

    use sea_orm_migration::MigratorTrait;

    use crate::{global::GLOBAL, cache::migration::CacheMigrator, data::migration::MainMigrator};

    use super::*;

    pub async fn get_or_init_temporary_main_database() -> &'static DatabaseConnection {
        GLOBAL.get_or_try_init_temporary_main_database(MainMigrator).await.unwrap()
    }
    pub async fn get_or_init_temporary_cache_database() -> &'static DatabaseConnection {
        GLOBAL.get_or_try_init_temporary_cache_database(CacheMigrator).await.unwrap()
    }

    #[tokio::test]
    async fn connect_main_database () {
        let db = get_or_init_temporary_main_database().await;
        assert!(db.ping().await.is_ok());
    }

    #[tokio::test]
    async fn connect_cache_database () {
        let db = get_or_init_temporary_cache_database().await;
        assert!(db.ping().await.is_ok());
    }
}