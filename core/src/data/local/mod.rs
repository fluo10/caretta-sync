mod authorization;
pub mod migration;

use std::{cell::OnceCell, iter::Map, path::Path, sync::{LazyLock, OnceLock}};

use migration::migrate;
use rusqlite::{ffi::Error, Connection, MappedRows, Row};

use crate::{config::StorageConfig, global::{CONFIG, LOCAL_DATABASE_CONNECTION}};

pub use authorization::*;

pub trait RusqliteRecord: Sized {
    fn insert(&self, connection: &Connection) -> Result<(), rusqlite::Error>;
    fn from_row(row: &Row<'_>) -> Result<Self, rusqlite::Error>;
    fn get_all(connection: &Connection) -> Result<Vec<Self>, rusqlite::Error>;
}

pub trait LocalRecord : RusqliteRecord{
    fn insert_global(&self) -> Result<(), rusqlite::Error> {
        self.insert(&LOCAL_DATABASE_CONNECTION.get_unchecked())
    }
    fn get_all_global() -> Result<Vec<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Self::get_all(&connection)
    }
}