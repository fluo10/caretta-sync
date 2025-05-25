use crate::config::{ServerConfig};
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

mod database;

pub static GLOBAL: Global = Global{
    server_config: OnceCell::const_new(),
    database: OnceCell::const_new(),
};
pub struct Global {
    server_config: OnceCell<ServerConfig>,
    database: OnceCell<DatabaseConnection>,
}
