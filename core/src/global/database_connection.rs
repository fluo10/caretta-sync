use std::{path::{Path, PathBuf}, sync::OnceLock};

use dirs::cache_dir;
use rusqlite::Connection;
use crate::{cache::migration::CacheMigrator, config::StorageConfig, data::local::migration::migrate, error::Error};
use tokio::sync::OnceCell;

pub static LOCAL_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new();

pub struct GlobalDatabaseConnection {
    inner: OnceLock<Connection>
}

impl GlobalDatabaseConnection {
    pub const fn const_new() -> Self {
        Self {
            inner: OnceLock::new()
        }
    }
    
    pub fn get(&'static self) -> Option<&'static Connection> {
        self.inner.get()
    }

    pub fn get_unchecked(&'static self) -> &'static Connection {
        self.get().expect("local data database connection should initialized before access!")
    }
    
    fn get_file_path<T>(config: &T) -> PathBuf 
    where 
        T: AsRef<StorageConfig>
    {
        config.as_ref().data_directory.join("local.sqlite")
    }

    pub async fn get_or_init_unchecked<T, U>(&'static self, config: T) -> Connection
    where
        T: AsRef<StorageConfig>,
    {
        let path = Self::get_file_path(&config);
        if let Some(x) = path.parent() {
            std::fs::create_dir_all(x).expect("Failed to create directory for local database");
        }
        self.inner.get_or_init(|| {
            let db = Connection::open(path)?;
            migrate(&db).unwrap();
            db
        })
        
    }
}

#[cfg(test)]
pub use tests::*;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cache::migration::CacheMigrator, global::CONFIG, tests::{TEST_CONFIG}};
    
    #[tokio::test]
    pub async fn get_or_init_database() {
        LOCAL_DATABASE_CONNECTION.get_or_init_unchecked(&*TEST_CONFIG).await;
    }
}
