use std::path::{Path, PathBuf};

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::{cache::migration::CacheMigrator, config::StorageConfig, error::Error};
use tokio::sync::OnceCell;

pub static DATABASE_CONNECTIONS: GlobalDatabaseConnections = GlobalDatabaseConnections::const_new();

pub struct DatabaseConnections<'a> {
    pub data: &'a DatabaseConnection,
    pub cache: &'a DatabaseConnection
}

pub struct GlobalDatabaseConnections {
    data: OnceCell<DatabaseConnection>,
    cache: OnceCell<DatabaseConnection>,
}

impl GlobalDatabaseConnections {
    pub const fn const_new() -> Self {
        Self {
            data: OnceCell::const_new(),
            cache: OnceCell::const_new()
        }
    }
    
    pub fn get_data(&'static self) -> Option<&'static DatabaseConnection> {
        self.data.get()
    }

    pub fn get_data_unchecked(&'static self) -> &'static DatabaseConnection {
        self.get_data().expect("Global data database connection should initialized before access!")
    }
    
    pub fn get_cache(&'static self) -> Option<&'static DatabaseConnection> {
        self.cache.get()
    }

    pub fn get_cache_unchecked(&'static self) -> &'static DatabaseConnection {
        self.get_cache().expect("Global cache database connection should initialized before access!")
    }
    
    fn get_data_file_path<T>(config: &T) -> PathBuf 
    where 
        T: AsRef<StorageConfig>
    {
        config.as_ref().data_directory.join("data.db")
    }
    
    fn get_cache_file_path<T>(config: &T) -> PathBuf 
    where 
        T: AsRef<StorageConfig>
    {
        config.as_ref().cache_directory.join("cache.db")
    }

    fn get_url_unchecked<T>(path: T) -> String
    where 
        T: AsRef<Path>
    {
        "sqlite://".to_string() + path.as_ref().as_os_str().to_str().expect("Failed to convert path to string!") + "?mode=rwc"
    }

    async fn get_or_init_database_connection_unchecked<T, U>(cell: &OnceCell<DatabaseConnection>, options: T, _: U ) -> &DatabaseConnection
    where 
        T: Into<ConnectOptions>,
        U: MigratorTrait
    {
        cell.get_or_init(|| async {
            let db = Database::connect(options.into()).await.unwrap();
            U::up(&db, None).await.unwrap();
            db
        }).await
    }


    pub async fn get_or_init_unchecked<T, U>(&'static self, config: T, _migrator: U) -> DatabaseConnections
    where
        T: AsRef<StorageConfig>,
        U: MigratorTrait,
    {
        DatabaseConnections{
            data: Self::get_or_init_database_connection_unchecked(
                &self.data,
                Self::get_url_unchecked(Self::get_data_file_path(&config)),
                _migrator
            ).await,
            cache: Self::get_or_init_database_connection_unchecked(
                &self.cache,
                Self::get_url_unchecked(Self::get_cache_file_path(&config)),
                CacheMigrator
            ).await,
        }
        
    }
}

#[cfg(test)]
pub use tests::*;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cache::migration::CacheMigrator, data::migration::DataMigrator, global::CONFIG, tests::{TEST_CONFIG}};
    
    #[tokio::test]
    pub async fn get_or_init_database() {
        DATABASE_CONNECTIONS.get_or_init_unchecked(&*TEST_CONFIG, DataMigrator).await;
    }
}
