use std::{marker::PhantomData, path::{Path, PathBuf}};

use crate::util::{Emptiable, Mergeable};

use caretta_sync_migration::Migrator;
use sea_orm::{Database, DatabaseConnection, sqlx::sqlite::SqliteConnectOptions};
use sea_orm_migration::MigratorTrait;
#[cfg(any(test, feature = "test"))]
use tempfile::tempdir;

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl StorageConfig {
    const LOCAL_DATABASE_FILE_NAME: &str = "local.db";
    const CACHE_DATABASE_FILE_NAME: &str = "cache.db";
    const IROH_DIR_NAME: &str = "iroh";

    pub fn to_iroh_path(&self) -> PathBuf {
        self.data_dir.join(Self::IROH_DIR_NAME)
    }
    // pub fn to_docs(&self) -> Result<Store, Error> {
    //     Ok(Store::persistent(self.to_docs_path()).map_err(|e| Error::DocsOpen(e))?)
    // }
    pub fn to_database_path(&self) -> PathBuf {
        self.data_dir().join(Self::LOCAL_DATABASE_FILE_NAME)
    }

    pub fn data_dir(&self) -> &Path{
        std::fs::create_dir_all(&self.data_dir).expect("Failed to create data dir");
        self.data_dir.as_path()
    }

    pub fn cache_dir(&self) -> &Path{
        std::fs::create_dir_all(&self.cache_dir).expect("Failed to create cache dir");
        self.cache_dir.as_path()
    }

    /// Open database for local data.
    ///
    /// # Panic
    /// If initialize database is failed, then panic.
    pub async fn to_database_connection(&self) -> DatabaseConnection {
        let options = ["sqlite://", &self.to_database_path().to_string_lossy(), "?mode=rwc"].join("");
        let db = Database::connect(&options).await.expect("Failed to open database file");
        Migrator::up(&db, None).await.expect("Failed to migrate database");
        db
    }
}
