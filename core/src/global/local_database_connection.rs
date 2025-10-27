use std::{fs::create_dir_all, path::Path};

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::sync::OnceCell;

use crate::error::Error;

pub static LOCAL_DATABASE_CONNECTION: GlobalDatabaseConnection =
    GlobalDatabaseConnection::const_new();

pub struct GlobalDatabaseConnection {
    inner: OnceCell<DatabaseConnection>,
}

impl GlobalDatabaseConnection {
    const fn const_new() -> Self {
        Self {
            inner: OnceCell::const_new(),
        }
    }

    pub async fn get_or_try_init<P, M>(&self, path: &P) -> Result<&DatabaseConnection, Error>
    where
        P: AsRef<Path>,
        M: MigratorTrait,
    {
        self.inner
            .get_or_try_init(|| async move {
                let path = path.as_ref();
                let parent = path
                    .parent()
                    .expect("Database path should have parent directory");
                create_dir_all(parent).expect("Failed to create parent directory of database");
                let url = "sqlite://".to_owned()
                    + path.to_str().expect("Invalid path string")
                    + "?mode=rwc";
                let db = Database::connect(url).await?;
                M::up(&db, None).await?;
                Ok(db)
            })
            .await
    }

    pub fn get(&self) -> Option<&DatabaseConnection> {
        self.inner.get()
    }
    pub fn get_unchecked(&self) -> &DatabaseConnection {
        self.get()
            .expect("Global database mulst be initialized before use")
    }
}
