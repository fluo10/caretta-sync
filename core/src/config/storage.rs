use std::{marker::PhantomData, path::PathBuf};

//use iroh_docs::store::Store;
use sea_orm::{Database, DatabaseConnection, sqlx::database};
use sea_orm_migration::MigratorTrait;

use crate::util::{Emptiable, Mergeable};

#[cfg(any(test, feature = "test"))]
use tempfile::tempdir;

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl StorageConfig {
    const DATABASE_FILE_NAME: &str = "database.sqlite";
    const DOCS_FILE_NAME: &str = "docs.bin";

    pub fn to_docs_path(&self) -> PathBuf {
        self.data_dir.join(Self::DOCS_FILE_NAME)
    }
    // pub fn to_docs(&self) -> Result<Store, Error> {
    //     Ok(Store::persistent(self.to_docs_path()).map_err(|e| Error::DocsOpen(e))?)
    // }
    pub fn to_database_path(&self) -> PathBuf {
        self.data_dir.join(Self::DATABASE_FILE_NAME)
    }

    /// Build database connection.
    ///
    /// # Panic
    /// If initialize database is failed, then panic.
    pub async fn to_database_connection<T>(&self, _: PhantomData<T>) -> DatabaseConnection
    where
        T: MigratorTrait,
    {
        let database_path = self.to_database_path();
        if let Some(x) = database_path.parent() {
            std::fs::create_dir_all(x).expect("Failed to create dir for database");
        }
        let url = "sqlite://".to_owned()
            + self
                .to_database_path()
                .to_str()
                .expect("Invalid path string")
            + "?mode=rwc";
        let db = Database::connect(url)
            .await
            .expect("Connecting database must be succeed.");
        T::up(&db, None)
            .await
            .expect("Database Migration must be succeed.");
        db
    }
}
