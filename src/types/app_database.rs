use std::path::Path;

use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;

use crate::types::util::path_to_sqlite_connect_options;
/// A wrapper stract of [`DatabaseConnection`] for application-side data.
/// 
/// This struct is used to distinguish between the application-side and caretta-sync library-side database.
#[derive(Debug)]
pub struct AppDatabase(DatabaseConnection);

impl AppDatabase {

    /// Open or create new `Database` with migration
    pub async fn open<P,M>(path: &P) -> Result<Self, DbErr> 
    where
        P: AsRef<Path>,
        M: MigratorTrait,
    {
        let db = sea_orm::Database::connect(path_to_sqlite_connect_options(path)).await?;
        M::up(&db, None).await?;
        Ok(Self(db))
    }
}

database_impl!(AppDatabase);