use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::{config::StorageConfig, error::Error};
use tokio::sync::OnceCell;

static UNINITIALIZED_MESSAGE: &str = "global storage is uninitialized!";

pub trait GlobalStorageConfig {

    fn init_storage_config(&'static self, config: StorageConfig) {
        self.get_storage_config_once_cell().set(config).unwrap();
    }
    fn get_storage_config_once_cell(&'static self) -> &'static OnceCell<StorageConfig>;
    fn get_storage_config(&'static self) -> Option<&'static StorageConfig> {
        self.get_storage_config_once_cell().get()
    }
    fn get_and_unwrap_storage_config(&'static self) -> &'static StorageConfig {
        self.get_storage_config().expect(UNINITIALIZED_MESSAGE)
    }
}