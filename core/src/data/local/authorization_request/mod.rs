//! Structs about authorization.

mod sent;
mod received;

use std::os::unix::raw::time_t;

use caretta_id::DoubleId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
pub use sent::*;
pub use received::*;
use rusqlite::{params, types::FromSqlError, Connection};
use uuid::Uuid;

use crate::data::local::LocalRecord;


/// Request of node authentication.
#[derive(Debug, Clone)]
pub struct AuthorizationRequestRecord {
    id: u32,
    uid: DoubleId,
    peer_id: u32,
    created_at: DateTime<Local>,
    closed_at: Option<DateTime<Local>>,
}

impl LocalRecord for AuthorizationRequestRecord {

    const TABLE_NAME: &str = "authorization_request";
    const SELECT_COLUMNS: &[&str] = &[
        "id",
        "uid",
        "peer_id",
        "created_at",
        "closed_at"
    ];
    const INSERT_COLUMNS: &[&str] = &[
        "uid",
        "peer_id",
        "created_at"
    ];
    
    type InsertParams<'a> = (&'a DoubleId, &'a [u8;32], &'a NaiveDateTime);  
    
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        let created_at: NaiveDateTime = row.get(3)?;
        let closed_at: Option<NaiveDateTime> = row.get(4)?;
        Ok(Self {
            id: row.get(0)?,
            uid: row.get(1)?,
            peer_id: row.get(2)?,
            created_at: created_at.and_utc().into(),
            closed_at: closed_at.map(|x| x.and_utc().into())
        })
    }
}