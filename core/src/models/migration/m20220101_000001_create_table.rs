use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DbBackend::Sqlite => {
                let db = manager.get_connection();
                db.execute_unprepared(
                    "CREATE TABLE invitation_token (
                        id             INTEGER PRIMARY KEY,
                        uuid           BLOB NOT NULL UNIQUE,
                        created_at     TEXT NOT NULL,
                        expires_at     TEXT NOT NULL,
                        used_at.       TEXT
                    )",
                )
                .await?;
                db.execute_unprepared(
                    "CREATE TABLE authorized_device (
                        id                       INTEGER PRIMARY KEY,
                        uuid                     BLOB UNIQUE NOT NULL,
                        public_id                INTEGER NOT NULL UNIQUE,
                        public_key               BLOB NOT NULL UNIQUE,
                        name                     TEXT NOT NULL,
                        created_at               TEXT NOT NULL,
                        updated_at               TEXT NOT NULL
                    )",
                )
                .await?;
                Ok(())
            }
            _ => Err(DbErr::Migration("Unsupported backend db".to_string())),
        }
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
