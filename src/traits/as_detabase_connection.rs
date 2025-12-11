use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub trait AsRef<DatabaseConnection> {
    fn as_database_connection(&self) -> &DatabaseConnection;
}

impl AsRef<DatabaseConnection> for DatabaseConnection {
    fn as_database_connection(&self) -> &DatabaseConnection {
        &self
    }
}

impl AsRef<DatabaseConnection> for Arc<DatabaseConnection> {
    fn as_database_connection(&self) -> &DatabaseConnection {
        self.as_ref()
    }
}