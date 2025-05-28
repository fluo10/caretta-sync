use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use crate::error::Error;
use tokio::sync::OnceCell;

use super::Global;

#[cfg(any(test, feature="test"))]
pub static TEST_DATABASE_URL: std::sync::LazyLock<tempfile::TempPath> = std::sync::LazyLock::new(|| {
    let mut temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
    temp_path.disable_cleanup(true);
    println!("{}", temp_path.as_os_str().to_str().unwrap());
    temp_path
});

impl Global {
    fn get_database(&self) -> Option<&DatabaseConnection> {
        self.database.get()
    }
    async fn get_or_try_init_database<T, U>(&self, path: T, _: U) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        U: MigratorTrait,
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";

        Ok(self.database.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            U::up(&db, None).await?;
            Ok::<DatabaseConnection, DbErr>(db)
        }).await?)
    }
    #[cfg(any(test, feature="test"))]
    pub async fn get_or_try_init_temporary_database<T>(&self, migrator: T) -> Result<&DatabaseConnection, Error>
    where 
        T: MigratorTrait,
    {
        self.get_or_try_init_database(&*TEST_DATABASE_URL, migrator).await
    }
}




#[cfg(test)]
pub mod tests {
    use std::sync::LazyLock;

    use lazy_supplements_migration::Migrator;
    use sea_orm_migration::MigratorTrait;

    use crate::global::GLOBAL;

    use super::*;

    pub async fn get_or_init_temporary_database() -> &'static DatabaseConnection {
        GLOBAL.get_or_try_init_temporary_database( Migrator).await.unwrap()
    }

    #[tokio::test]
    async fn connect_database () {
        let db = get_or_init_temporary_database().await;
        assert!(db.ping().await.is_ok());
    }

}