use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

static DATA_DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
static CACHE_DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub fn get_data_database_connection() -> Option<&'static DatabaseConnection> {
    DATA_DATABASE_CONNECTION.get()
}
pub fn get_and_unwrap_data_database_connection() -> &'static DatabaseConnection {
    get_data_database_connection().expect("global data database connection uninitialized!")
}
pub async fn get_or_try_init_data_database_connection<T, U>(path: T, _: U) -> Result<&'static DatabaseConnection, Error>
where
    T: AsRef<Path>,
    U: MigratorTrait
{
    let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";
    Ok(DATA_DATABASE_CONNECTION.get_or_try_init(|| async {
        let db = Database::connect(&url).await?;
        U::up(&db, None).await?;
        Ok::<DatabaseConnection, DbErr>(db)
    }).await?)
}

pub fn get_cache_database_connection() -> Option<&'static DatabaseConnection> {
    CACHE_DATABASE_CONNECTION.get()
}
pub fn get_and_unwrap_cache_database_connection() -> &'static DatabaseConnection {
    CACHE_DATABASE_CONNECTION.get().expect("global data database connection uninitialized!")
}
pub async fn get_or_try_init_cache_database_connection<T, U>(path: T, _: U) -> Result<&'static DatabaseConnection, Error>
where
    T: AsRef<Path>,
    U: MigratorTrait
{
    let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";
    Ok(DATA_DATABASE_CONNECTION.get_or_try_init(|| async {
        let db = Database::connect(&url).await?;
        U::up(&db, None).await?;
        Ok::<DatabaseConnection, DbErr>(db)
    }).await?)
}