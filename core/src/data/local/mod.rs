mod authorization_request;
mod peer;
pub mod migration;

use std::{cell::OnceCell, iter::Map, path::Path, sync::{LazyLock, OnceLock}};

use migration::migrate;
use rusqlite::{ffi::Error, Connection, MappedRows, Row};

use crate::{config::StorageConfig, global::{CONFIG, LOCAL_DATABASE_CONNECTION}};

pub use authorization_request::*;

/// Model trait for local database data.
/// use LOCAL_DATABASE_CONNECTION for database connection.
pub trait LocalModel: Sized {
    const TABLE_NAME: &str;
    const DEFAULT_COLUMNS: &[&str];
    fn insert(&self) -> Result<(), rusqlite::Error>;
    fn from_default_row(row: &Row<'_>) -> Result<Self, rusqlite::Error>;
    fn get_all() -> Result<Vec<Self>, rusqlite::Error>;
}