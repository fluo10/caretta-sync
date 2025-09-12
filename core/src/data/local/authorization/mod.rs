//! Structs about authorization.

mod request;
mod response;

use std::os::unix::raw::time_t;

use chrono::{DateTime, Local, NaiveDateTime};
use iroh::NodeId;
pub use request::*;
pub use response::*;
use rusqlite::{params, types::FromSqlError, Connection};
use uuid::Uuid;

use crate::data::local::RusqliteRecord;

/// On going authorization
pub struct Authorization {
    request_id: Uuid,
    node_id: NodeId,
    node_info: Option<String>,
    passcode: Option<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
static TABLE_NAME: &str = "authorization";
static DEFAULT_COLUMNS: [&str;5] = [
    "request_id",
    "node_id",
    "created_at",
    "updated_at"
];

impl Authorization {
    pub fn new_sent(node_id: NodeId, passcode: String) -> Self {
        let timestamp = Local::now();
        Self {
            node_id: node_id,
            passcode: passcode,
            created_at: timestamp.clone(),
            updated_at: timestamp
        }
    }
    pub fn new_received(node_id:)
    pub fn get_by_node_id(node_id: NodeId, connection: &Connection) -> Result<Self, rusqlite::Error> {
        connection.query_row(
            "SELECT node_id, passcode, created_at, updated_at FROM authorizaation WHRE node_id=(?1)",
            params![node_id.as_bytes()],
            Self::from_row
        )
    }


}
impl RusqliteRecord for Authorization {
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        let created_at: NaiveDateTime = row.get(2)?;
        let updated_at: NaiveDateTime = row.get(3)?;
        let node_id: Vec<u8> = row.get(0)?;
        Ok(Self {
            node_id: NodeId::from_bytes(node_id[..32].try_into().or_else(|e| {
                Err(rusqlite::types::FromSqlError::InvalidBlobSize {
                    expected_size: 32,
                    blob_size: node_id.len()
                })
            })?).or(Err(FromSqlError::InvalidType))?,
            passcode: row.get(1)?,
            created_at: DateTime::from(created_at.and_utc()),
            updated_at: DateTime::from(updated_at.and_utc()),
        })
    }
    fn insert(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO authorization (node_id, passcode, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (&self.node_id.as_bytes(), &self.passcode, &self.created_at.naive_utc(), &self.updated_at.naive_utc()),
        )?;
        Ok(())
    }
    fn get_all(connection: &rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = connection.prepare(&(String::from("SELECT ") + &DEFAULT_COLUMNS.join(", ") + " FROM " + TABLE_NAME))?;
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