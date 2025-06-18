use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

use super::storage_config::GlobalStorageConfig;

static UNINITIALIZED_MESSAGE: &str = "global database connection uninitialized!";

pub trait GlobalDatabaseConnection: GlobalStorageConfig {
    fn get_data_database_connection_as_once_cell(&'static self) -> &'static OnceCell<DatabaseConnection>;
    fn get_data_database_connection(&'static self) -> Option<&'static DatabaseConnection> {
        self.get_data_database_connection_as_once_cell().get()
    }
    fn get_and_unwrap_data_database_connection(&'static self) -> &'static DatabaseConnection {
        self.get_data_database_connection().expect(UNINITIALIZED_MESSAGE)
    }
    async fn get_or_try_init_data_database_connection<T>(&'static self, _: T) -> Result<&DatabaseConnection, Error>
    where
        T: MigratorTrait
    {
        let url = "sqlite://".to_string() + self.get_and_unwrap_storage_config().get_data_database_path().to_str().unwrap() + "?mode=rwc";
        Ok(self.get_data_database_connection_as_once_cell().get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            T::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }
    
}