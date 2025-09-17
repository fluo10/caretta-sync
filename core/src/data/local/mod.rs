mod authorization_request;
mod peer;
pub mod migration;

use std::{cell::OnceCell, iter::Map, path::Path, sync::{LazyLock, OnceLock}};

use migration::migrate;
use rusqlite::{ffi::Error, params, Connection, MappedRows, OptionalExtension, Params, Row, ToSql};

use crate::{config::StorageConfig, global::{CONFIG, LOCAL_DATABASE_CONNECTION}};

pub use authorization_request::*;

/// Model trait for local database data.
/// use LOCAL_DATABASE_CONNECTION for database connection.
pub trait LocalRecord: Sized {
    const TABLE_NAME: &str;
    const DEFAULT_COLUMNS: &[&str];

    const DEFAULT_SELECT_STATEMENT: LazyLock<String> = LazyLock::new(|| {
        String::from("SELECT ") + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME
    });

    const DEFAULT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
        let mut result : Vec<String> = Vec::new();
        for i in 0..Self::DEFAULT_COLUMNS.len() {
            result.push(String::from("?") + &(i+1).to_string());
        }
        result.join(", ")
    });

    type DefaultParams<'a>: Params
    where 
        Self: 'a;
    
    fn as_default_params<'a>(&'a self) -> Self::DefaultParams<'a>;

    fn insert(&self) -> Result<(), rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        
        connection.execute(
            &("INSERT INTO ".to_owned() + Self::TABLE_NAME + " (" + &Self::DEFAULT_COLUMNS.join(", ") + ") VALUES (" + &*Self::DEFAULT_PLACEHOLDER + ")"),
            self.as_default_params()
        )?;
        Ok(())
    }
    
    fn get_one_where<P>(where_statement: &str, params: P) -> Result<Option<Self>, rusqlite::Error> 
    where P: Params
    {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(connection.query_row(
            &(String::new() + &Self::DEFAULT_SELECT_STATEMENT + " " + where_statement),
            params,
            Self::from_default_row
        ).optional()?)
    }

    fn get_one_by<T>(field_name: &str, field_value: T) -> Result<Option<Self>, rusqlite::Error> 
    where 
        T: ToSql
    {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(Some(connection.query_row(
            &("SELECT ".to_string() + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME + " WHERE " + field_name + "=(?1)"),
            params![field_value],
            Self::from_default_row
        )?))
    }
    fn get_one_by_id(id: u32) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_by("id", id )
    }
    fn from_default_row(row: &Row<'_>) -> Result<Self, rusqlite::Error>;
    fn get_all() -> Result<Vec<Self>, rusqlite::Error>;
}