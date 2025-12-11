use std::{marker::PhantomData, path::{Path, PathBuf}};

use crate::util::{Emptiable, Mergeable};

use redb::Database;
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
    const DOCS_FILE_NAME: &str = "docs.bin";

    pub fn to_docs_path(&self) -> PathBuf {
        self.data_dir.join(Self::DOCS_FILE_NAME)
    }
    // pub fn to_docs(&self) -> Result<Store, Error> {
    //     Ok(Store::persistent(self.to_docs_path()).map_err(|e| Error::DocsOpen(e))?)
    // }
    pub fn to_local_database_path(&self) -> PathBuf {
        self.data_dir().join(Self::LOCAL_DATABASE_FILE_NAME)
    }
    pub fn to_cache_database_path(&self) -> PathBuf {
        self.cache_dir().join(Self::CACHE_DATABASE_FILE_NAME)
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
    pub fn to_local_database(&self) -> Database
    {
        Database::create(self.to_local_database_path()).expect("Failed to open local database")
    }
    /// Open database for cache data.
    ///
    /// # Panic
    /// If initialize database is failed, then panic.
    pub fn to_cache_database(&self) -> Database
    {
        Database::create(self.to_cache_database_path()).expect("Failed to open local database")
    }
}
