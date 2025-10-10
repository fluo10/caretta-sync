use sea_orm_migration::prelude::*;
use sea_orm::DbBackend;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DbBackend::Sqlite => {
                let db = manager.get_connection();
                db.execute_unprepared(
                    "CREATE TABLE remote_node (
                        id         INTEGER PRIMARY KEY,
                        public_id  INTEGER NOT NULL UNIQUE,
                        public_key BLOB UNIQUE NOT NULL
                    )"
                ).await?;
                db.execute_unprepared(
                    "CREATE TABLE authorization_request (
                        id             INTEGER PRIMARY KEY,
                        uuid           BLOB NOT NULL UNIQUE,
                        public_id      INTEGER NOT NULL UNIQUE,
                        remote_node_id INTEGER NOT NULL UNIQUE,
                        created_at     TEXT NOT NULL,
                        closed_at      TEXT,
                        FOREIGN KEY(remote_node_id) REFERENCES remote_node(id)
                    )"
                ).await?;
                db.execute_unprepared(
                    "CREATE TABLE received_authorization_request (
                        id                       INTEGER PRIMARY KEY,
                        authorization_request_id INTEGER NOT NULL UNIQUE,
                        sender_note         TEXT,
                        FOREIGN KEY(authorization_request_id) REFERENCES authorization_request(id)
                    )"
                ).await?;
                db.execute_unprepared(
                    "CREATE TABLE sent_authorization_request (
                        id                       INTEGER PRIMARY KEY,
                        authorization_request_id INTEGER NOT NULL UNIQUE,
                        passcode                 INTEGER NOT NULL,
                        FOREIGN KEY(authorization_request_id) REFERENCES authorization_request(id)
                    )"
                ).await?;
                db.execute_unprepared(
                    "CREATE TABLE authorized_remote_node (
                        id                       INTEGER PRIMARY KEY,
                        uuid                     BLOB UNIQUE NOT NULL,
                        public_id                INTEGER NOT NULL UNIQUE,
                        public_key               BLOB NOT NULL UNIQUE,
                        note                     TEXT NOT NULL,
                        last_synced_at           TEXT,
                        last_sent_version_vector BLOB,
                        created_at               TEXT NOT NULL,
                        updated_at               TEXT NOT NULL
                    )"
                ).await?;
                Ok(())
            },
            _ => Err(DbErr::Migration("Unsupported backend db".to_string()))
        }        
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}