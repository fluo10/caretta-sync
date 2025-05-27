use std::path::Path;

use sea_orm::{ConnectOptions, Database, DbErr, DatabaseConnection};
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
    async fn get_or_try_init_database<T, F, Fut>(&self, path: T, migration: F) -> Result<&DatabaseConnection, Error>
    where
        T: AsRef<Path>,
        F: FnOnce(DatabaseConnection) -> Fut,
        Fut: Future<Output = Result<DatabaseConnection, DbErr>>
    {
        let url = "sqlite://".to_string() + path.as_ref().to_str().unwrap() + "?mode=rwc";

        Ok(self.database.get_or_try_init(|| async {
            let db = Database::connect(&url).await?;
            Ok::<DatabaseConnection, DbErr>(migration(db).await?)
        }).await?)
    }
    #[cfg(any(test, feature="test"))]
    pub async fn get_or_try_init_temporary_database<F, Fut>(&self, migration: F) -> Result<&DatabaseConnection, Error>
    where 
        F: FnOnce(DatabaseConnection) -> Fut,
        Fut: Future<Output = Result<DatabaseConnection, DbErr>>
    {
        self.get_or_try_init_database(&*TEST_DATABASE_URL, migration).await
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
        GLOBAL.get_or_try_init_temporary_database( |x| async {
            Migrator::up(&x, None).await?;
            Ok(x)
        }).await.unwrap()
    }

    #[tokio::test]
    async fn connect_database () {
        let db = get_or_init_temporary_database().await;
        assert!(db.ping().await.is_ok());
    }

}