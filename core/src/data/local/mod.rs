// mod authorization_request;
mod remote_node;
pub mod migration;

use std::{cell::OnceCell, convert::Infallible, iter::Map, path::Path, sync::{LazyLock, OnceLock}};

use migration::migrate;
use rusqlite::{ffi::Error, params, types::FromSql, Connection, MappedRows, OptionalExtension, Params, Row, ToSql};

use crate::{config::StorageConfig, global::{CONFIG, LOCAL_DATABASE_CONNECTION}};

// pub use authorization_request::*;
type LocalRecordError = rusqlite::Error;


/// a struct of id for local database record.
pub type LocalRecordId = u32;

/// a struct for the record without id before inserted
pub type NoLocalRecordId = rusqlite::types::Null;



/// A id struct
/// Model trait for local database data.
/// use LOCAL_DATABASE_CONNECTION for database connection.
pub trait LocalRecord: Sized {
    const TABLE_NAME: &str;
    const COLUMNS: &[&str];

    /// Tuple form of the record.
    /// the order of field must be same as COLUMNS.
    type RowValues;
}
pub trait SelectableLocalRecord: LocalRecord<RowValues: TryInto<Self>> {

    const SELECT_STATEMENT: LazyLock<String> = LazyLock::new(|| {
        String::from("SELECT ") + &Self::COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME
    });

    const SELECT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
        let mut result : Vec<String> = Vec::new();
        for i in 0..Self::COLUMNS.len() {
            result.push(String::from("?") + &(i+1).to_string());
        }
        result.join(", ")
    });


    
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
            &("SELECT ".to_string() + &Self::COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME + " WHERE " + field_name + "= ?1"),
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
        let mut stmt = connection.prepare(&("SELECT ".to_string() + &Self::COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME))?;
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

pub trait InsertableLocalRecord: LocalRecord<RowValues: From<Self> + Params> {
    type LocalRecord: Sized + SelectableLocalRecord;

    /// Place holder for insertion.
    /// Generated from Columns
    const INSERT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
        let mut result : Vec<String> = Vec::new();
        for i in 0..Self::COLUMNS.len() {
            result.push(String::from("?") + &(i+1).to_string());
        }
        result.join(", ")
    });    
    /// Insert and get the inserted record.
    fn insert(self) -> Result<Self::LocalRecord, rusqlite::Error>{
        let params= Self::RowValues::from(self);
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        
        Ok(connection.query_row(
            &[
                "INSERT INTO ", Self::TABLE_NAME,  "(" , &Self::COLUMNS.join(", "),  ")", 
                "VALUES (" , &*Self::INSERT_PLACEHOLDER , ")",
                "RETURNING", &Self::COLUMNS.join(", ")
            ].join(" "),
            params,
            Self::LocalRecord::from_row
        )?)
    }
}