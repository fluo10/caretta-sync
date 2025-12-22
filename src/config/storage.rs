use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{
    types::{AppDatabase, Database},
    util::{Emptiable, Mergeable},
};

use caretta_sync_migration::Migrator;
use sea_orm::{DatabaseConnection, sqlx::sqlite::SqliteConnectOptions};
use sea_orm_migration::MigratorTrait;
#[cfg(any(test, feature = "test"))]
use tempfile::tempdir;

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl StorageConfig {
    const DATABASE_FILE_NAME: &str = "caretta-sync.sqlite";
    const IROH_DIR_NAME: &str = "iroh";

    pub fn to_iroh_path(&self) -> PathBuf {
        self.data_dir.join(Self::IROH_DIR_NAME)
    }
    // pub fn to_docs(&self) -> Result<Store, Error> {
    //     Ok(Store::persistent(self.to_docs_path()).map_err(|e| Error::DocsOpen(e))?)
    // }
    pub fn to_database_path(&self) -> PathBuf {
        self.data_dir().join(Self::DATABASE_FILE_NAME)
    }
    pub fn to_app_database_path(&self, app_name: &'static str) -> PathBuf {
        self.data_dir().join(format!("{app_name}.sqlite"))
    }

    pub fn data_dir(&self) -> &Path {
        std::fs::create_dir_all(&self.data_dir).expect("Failed to create data dir");
        self.data_dir.as_path()
    }

    pub fn cache_dir(&self) -> &Path {
        std::fs::create_dir_all(&self.cache_dir).expect("Failed to create cache dir");
        self.cache_dir.as_path()
    }

    /// Open database for local data.
    ///
    /// # Panic
    /// If initialize database is failed, then panic.
    pub async fn open_database(&self) -> Database {
        Database::open(&self.to_database_path()).await.unwrap()
    }
    /// Open database for application side data.
    ///
    /// # Panic
    /// If initialize database is failed, then panic.
    pub async fn open_app_database<M>(&self, app_name: &'static str) -> AppDatabase
    where
        M: MigratorTrait,
    {
        AppDatabase::open::<_, M>(&self.to_app_database_path(app_name))
            .await
            .unwrap()
    }
}
