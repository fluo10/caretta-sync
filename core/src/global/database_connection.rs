use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

pub static DATA_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new(stringify!(DATA_DATABASE_CONNECTION));
pub static CACHE_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new(stringify!(CACHE_DATABASE_CONNECTION));

pub struct GlobalDatabaseConnection {
    name: &'static str,
    inner: OnceCell<DatabaseConnection>
}

impl GlobalDatabaseConnection {
    pub const fn const_new(name: &'static str) -> Self {
        Self {
            name: name,
            inner: OnceCell::const_new()
        }
    }
    pub fn get(&'static self) -> &'static DatabaseConnection {
        self.inner.get().expect(&format!("{} is uninitialized!", self.name))
    }
    pub async fn get_or_init<T, U>(&'static self, path: T, _: U) -> &'static DatabaseConnection
    where
        T: AsRef<Path>,
        U: MigratorTrait
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";
        self.inner.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await.expect(&format!("Fail to initialize {}!", self.name))
    }
}

#[cfg(test)]
pub use tests::*;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cache::migration::CacheMigrator, data::migration::DataMigrator, global::STORAGE_CONFIG, tests::GlobalTestDefault};
    
    pub async fn get_or_init_test_data_database() -> &'static DatabaseConnection{
        DATA_DATABASE_CONNECTION.get_or_init(STORAGE_CONFIG.get_or_init_test_default().await.get_data_database_path(), DataMigrator).await
    }
    pub async fn get_or_init_test_cache_database() -> &'static DatabaseConnection{
        CACHE_DATABASE_CONNECTION.get_or_init(STORAGE_CONFIG.get_or_init_test_default().await.get_cache_database_path(), CacheMigrator).await
    }
}