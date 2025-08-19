use std::path::{Path, PathBuf};

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::{config::StorageConfig, error::Error};
use tokio::sync::OnceCell;

enum StorageType {
    Data,
    Cache,
}

impl std::fmt::Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", match self {
            StorageType::Data => "data",
            StorageType::Cache => "cache",
        });
        Ok(())
    }
}


pub static DATA_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new(StorageType::Data);
pub static CACHE_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new(StorageType::Cache);

pub struct GlobalDatabaseConnection {
    storage: StorageType,
    inner: OnceCell<DatabaseConnection>
}

impl GlobalDatabaseConnection {
    pub const fn const_new(storage: StorageType) -> Self {
        Self {
            storage: storage,
            inner: OnceCell::const_new()
        }
    }
    pub fn get(&'static self) -> Option<&'static DatabaseConnection> {
        self.inner.get()
    }
    fn get_file_path<T>(&self, config: T) -> PathBuf 
    where 
        T: AsRef<StorageConfig>
    {
        match self.storage {
            StorageType::Cache => config.as_ref().cache_directory.join("cache.db"),
            StorageType::Data => config.as_ref().data_directory.join("data.db"),
        }
    }
    pub fn get_unchecked(&'static self) -> &'static DatabaseConnection {
        self.get().expect("Global database connection should initialized beforehand!")
    }
    pub async fn get_or_try_init<T, U>(&'static self, config: T, _: U) -> Result<&'static DatabaseConnection, Error>
    where
        T: AsRef<StorageConfig>,
        U: MigratorTrait
    {
        let url = "sqlite://".to_string() + &self.get_file_path(config).into_os_string().into_string()? + "?mode=rwc";
        Ok(self.inner.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }
}

#[cfg(test)]
pub use tests::*;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cache::migration::CacheMigrator, data::migration::DataMigrator, global::CONFIG, tests::{TEST_CONFIG}};
    
    pub async fn get_or_init_test_data_database() -> &'static DatabaseConnection{
        DATA_DATABASE_CONNECTION.get_or_try_init(&*TEST_CONFIG, DataMigrator).await.unwrap()
    }
    pub async fn get_or_init_test_cache_database() -> &'static DatabaseConnection{
        CACHE_DATABASE_CONNECTION.get_or_try_init(&*TEST_CONFIG, CacheMigrator).await.unwrap()
    }
}