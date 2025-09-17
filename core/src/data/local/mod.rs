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
    const SELECT_COLUMNS: &[&str];
    const INSERT_COLUMNS: &[&str];

    const SELECT_STATEMENT: LazyLock<String> = LazyLock::new(|| {
        String::from("SELECT ") + &Self::SELECT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME
    });

    const SELECT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
        let mut result : Vec<String> = Vec::new();
        for i in 0..Self::SELECT_COLUMNS.len() {
            result.push(String::from("?") + &(i+1).to_string());
        }
        result.join(", ")
    });
    const INSERT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
        let mut result : Vec<String> = Vec::new();
        for i in 0..Self::INSERT_COLUMNS.len() {
            result.push(String::from("?") + &(i+1).to_string());
        }
        result.join(", ")
    });

    type InsertParams<'a>: Params
    where 
        Self: 'a;
    
    fn insert(params: Self::InsertParams<'_>) -> Result<Self, rusqlite::Error>
     {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        
        Ok(connection.query_row(
            &[
                "INSERT INTO ", Self::TABLE_NAME,  "(" , &Self::INSERT_COLUMNS.join(", "),  ")", 
                "VALUES (" , &*Self::INSERT_PLACEHOLDER , ")",
                "RETURNING", &Self::SELECT_COLUMNS.join(", ")
            ].join(" "),
            params,
            Self::from_row
        )?)
    }
    
    fn get_one_where<P>(where_statement: &str, params: P) -> Result<Option<Self>, rusqlite::Error> 
    where P: Params
    {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(connection.query_row(
            &(String::new() + &Self::SELECT_STATEMENT + " " + where_statement),
            params,
            Self::from_row
        ).optional()?)
    }

    fn get_one_by_field<T>(field_name: &str, field_value: T) -> Result<Option<Self>, rusqlite::Error> 
    where 
        T: ToSql
    {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(Some(connection.query_row(
            &("SELECT ".to_string() + &Self::SELECT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME + " WHERE " + field_name + "= ?1"),
            params![field_value],
            Self::from_row
        )?))
    }
    fn get_one_by_id(id: u32) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_by_field("id", id )
    }
    fn from_row(row: &Row<'_>) -> Result<Self, rusqlite::Error>;
    fn get_all() -> Result<Vec<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        let mut stmt = connection.prepare(&("SELECT ".to_string() + &Self::SELECT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME))?;
        let rows = stmt.query_map(
            [],
            Self::from_row
        )?;
        let mut result= Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    
    }
}