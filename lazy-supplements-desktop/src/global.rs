use std::{path::PathBuf, sync::LazyLock};

use lazy_supplements_core::config::PartialCoreConfig;
pub use lazy_supplements_core::global::*;

pub static DEFAULT_DATA_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::data_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});
pub static DEFAULT_CONFIG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = if let Some(x) = dirs::config_local_dir() {
        x
    } else {
        todo!()
    };
    
    dir.join(&*PRODUCT_NAME)
});

pub static DEFAULT_CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_CONFIG_DIR_PATH.join(&*DEFAULT_CONFIG_FILE_NAME)
});
pub static DEFAULT_DATABASE_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DEFAULT_DATA_DIR_PATH.join(&*DEFAULT_DATABASE_FILE_NAME)
});

pub static DEFAULT_PARTIAL_CORE_CONFIG: LazyLock<PartialCoreConfig> = LazyLock::new(|| {
    PartialCoreConfig {
        secret: None,
        listen_ips: Some(DEFAULT_LISTEN_IPS.to_vec()),
        port: Some(0),
    }
});

pub struct Global {
    pub p2p_config: OnceCell<P2pConfig>,
    pub main_database: OnceCell<DatabaseConnection>,
    pub cache_database: OnceCell<DatabaseConnection>,
    pub peers: OnceCell<RwLock<HashMap<PeerId, Multiaddr>>>,
}

impl Global {
    pub fn get_p2p_config(&self) -> Option<&P2pConfig> {
        self.p2p_config.get()
    }
    pub async fn get_or_init_p2p_config(&self, config: P2pConfig) -> &P2pConfig {
        self.p2p_config.get_or_init(|| async {config}).await
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
        let mut swarm = self.get_p2p_config().unwrap().clone().try_into_swarm().await?;
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