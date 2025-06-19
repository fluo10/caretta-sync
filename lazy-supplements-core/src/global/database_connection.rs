use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

static DATA_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new();
static CACHE_DATABASE_CONNECTION: GlobalDatabaseConnection = GlobalDatabaseConnection::const_new();

struct GlobalDatabaseConnection {
    inner: OnceCell<DatabaseConnection>
}

impl GlobalDatabaseConnection {
    pub const fn const_new() -> Self {
        Self {
            inner: OnceCell::const_new()
        }
    }
    pub fn get(&'static self) -> Option<&'static DatabaseConnection> {
        self.inner.get()
    }
    pub fn get_and_unwrap(&'static self) -> &'static DatabaseConnection {
        self.get().expect(&format!("{} is uninitialized!", &stringify!(self)))
    }
    pub async fn get_or_try_init<T, U>(&'static self, path: T, _: U) -> Result<&'static DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";
        Ok(self.inner.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }
}
