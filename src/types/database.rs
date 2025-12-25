use std::path::Path;

use caretta_framework_migration::Migrator;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;

use crate::types::util::path_to_sqlite_connect_options;
/// A wrapper stract of [`DatabaseConnection`] for `caretta-framework`.
///
/// This struct is used to distinguish between the application-side and caretta-framework library-side database.
#[derive(Debug)]
pub struct Database(DatabaseConnection);

impl Database {
    /// Open or create new `Database`
    pub async fn open<P>(path: &P) -> Result<Self, DbErr>
    where
        P: AsRef<Path>,
    {
        let db = sea_orm::Database::connect(path_to_sqlite_connect_options(path)).await?;
        Migrator::up(&db, None).await?;
        Ok(Self(db))
    }
}

database_impl!(Database);
