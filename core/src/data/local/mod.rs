pub mod migration;

use std::{cell::OnceCell, path::Path, sync::{LazyLock, OnceLock}};

use migration::migrate;
use rusqlite::{ffi::Error, Connection};

use crate::{config::StorageConfig, global::CONFIG};

static INITIALIZE_PARENT_DIRECTORY_RESULT: OnceLock<()> = OnceLock::new();

static MIGRATE_RESULT: OnceLock<()> = OnceLock::new();

fn initialize_parent_directory<P>(path: &P)
where 
    P: AsRef<Path>,
{
    *INITIALIZE_PARENT_DIRECTORY_RESULT.get_or_init(|| {
        let path2: &Path = path.as_ref();
        if let Some(x) = path2.parent() {
            if !x.exists() {
                std::fs::create_dir_all(x).expect("Parent directory of the local database must be created.");
            }
        }
    })
}

fn migrate_once(conn: &mut Connection) -> () {
    *MIGRATE_RESULT.get_or_init(|| {
        migrate(conn).expect("Local database migration should be done correctly")
    })

}
pub trait LocalDatabaseConnection: Sized {
    fn from_path<P>(path: &P) -> Self
    where 
        P: AsRef<Path>;
    fn from_storage_config(config: &StorageConfig) -> Self {
        Self::from_path(&config.get_local_database_path())
    }
    fn from_global_storage_config() -> Self {
        Self::from_storage_config(&CONFIG.get_unchecked().storage)
    }
}

impl LocalDatabaseConnection for Connection {
    fn from_path<P>(path: &P) -> Self
    where 
        P: AsRef<Path>
    {
        initialize_parent_directory(path);
        let mut conn = Connection::open(path).expect("local database connection must be opened without error");
        migrate_once(&mut conn);
        conn        
    }
}