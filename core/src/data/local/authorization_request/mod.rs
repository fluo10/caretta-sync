//! Structs about authorization.

mod sent;
mod received;

use std::os::unix::raw::time_t;

use tripod_id::Double;
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
    uuid: Uuid,
    public_id: Double,
    peer_id: u32,
    created_at: DateTime<Local>,
    closed_at: Option<DateTime<Local>>,
}

impl LocalRecord for AuthorizationRequestRecord {

    const TABLE_NAME: &str = "authorization_request";
    const SELECT_COLUMNS: &[&str] = &[
        "id",
        "uuid",
        "public_id",
        "peer_id",
        "created_at",
        "closed_at"
    ];
    const INSERT_COLUMNS: &[&str] = &[
        "uuid",
        "public_id",
        "peer_id",
        "created_at"
    ];
    
    type InsertParams<'a> = (&'a Double, &'a [u8;32], &'a NaiveDateTime);  
    type SelectValues = (u32, Uuid, Double, PublicKey, NaiveDateTime, Option<NaiveDateTime>);
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        let created_at: NaiveDateTime = row.get(4)?;
        let closed_at: Option<NaiveDateTime> = row.get(5)?;
        Ok(Self {
            id: row.get(0)?,
            uuid: row.get(1)?,
            public_id: row.get(2)?,
            peer_id: row.get(3)?,
            created_at: created_at.and_utc().into(),
            closed_at: closed_at.map(|x| x.and_utc().into())
        })
    }
}

impl From<(u32, Uuid, Double, u32, NaiveDateTime, Option<NaiveDateTime>)> for AuthorizationRequestRecord {
    fn from(value: (u32, Uuid, Double, u32, NaiveDateTime, Option<NaiveDateTime>)) -> Self {
        Self {
            id: value.0,
            uuid: value.1,
            public_id: value.2,
            peer_id: value.3,
            created_at: value.4.and_utc().into(),
            closed_at: value.5.map(|x| x.and_utc().into())
        }
    }
}
impl<'a> From<&'a rusqlite::Row<'_>> for AuthorizationRequestRecord {
    fn from(value: &'a rusqlite::Row<'_>) -> Self {
        todo!()
    }
}
