use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

use super::Global;

#[cfg(any(test, feature="test"))]
pub static TEST_MAIN_DATABASE_URL: std::sync::LazyLock<tempfile::TempPath> = std::sync::LazyLock::new(|| {
    let mut temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
    temp_path.disable_cleanup(true);
    println!("{}", temp_path.as_os_str().to_str().unwrap());
    temp_path
});
#[cfg(any(test, feature="test"))]
pub static TEST_CACHE_DATABASE_URL: std::sync::LazyLock<tempfile::TempPath> = std::sync::LazyLock::new(|| {
    let mut temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
    temp_path.disable_cleanup(true);
    println!("{}", temp_path.as_os_str().to_str().unwrap());
    temp_path
});

pub trait GlobalDatabase {
    fn get_main_database(&self) -> Option<&DatabaseConnection>;
    async fn get_or_try_init_main_database<T, U>(&self, path: T, _: U) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait
    ;
    fn get_unwrapped_main_database(&self) -> &DatabaseConnection {
        match self.get_main_database() {
            Some(x) => x,
            None => unreachable!("Error: global main database is not initialized!")
        }
    }
    fn get_cache_database(&self) -> Option<&DatabaseConnection>;
    async fn get_or_try_init_cache_database<T, U>(&self, path: T, _: U) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait
    ;
    fn get_unwrapped_cache_database(&self) -> &DatabaseConnection {
        match self.get_cache_database() {
            Some(x) => x,
            None => unreachable!("Error: global main database is not initialized!")
        }
    }
    #[cfg(any(test, feature="test"))]
    async fn get_or_try_init_temporary_main_database<T>(&self, migrator: T) -> Result<&DatabaseConnection, Error>
    where 
        T: MigratorTrait,
    {
        self.get_or_try_init_main_database(&*TEST_MAIN_DATABASE_URL, migrator).await
    }
    #[cfg(any(test, feature="test"))]
    async fn get_or_try_init_temporary_cache_database<T>(&self, migrator: T) -> Result<&DatabaseConnection, Error>
    where 
        T: MigratorTrait,
    {
        self.get_or_try_init_cache_database(&*TEST_CACHE_DATABASE_URL, migrator).await
    }
}