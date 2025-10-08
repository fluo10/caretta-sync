use caretta_id::StidId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::types::FromSqlError;

use crate::{data::local::LocalRecord, global::LOCAL_DATABASE_CONNECTION};

/// Request of node authentication.
#[derive(Debug, Clone)]
pub struct SentAuthorizationRequestRecord {
    id: u32,
    authorization_request_id: u32,
    passcode: String,
}

impl LocalRecord for SentAuthorizationRequestRecord {

    const TABLE_NAME: &str = "sent_authorization_request";
    const SELECT_COLUMNS: &[&str] = &[
        "id",
        "authorization_request_id",
        "passcode",
    ];
    const INSERT_COLUMNS: &[&str] = &[
        "authorization_request_id",
        "passcode"
    ];
    
    type InsertParams<'a> = (&'a u32, &'a str);
    
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self{
            id: row.get(0)?,
            authorization_request_id: row.get(0)?,
            passcode: row.get(2)?
        })
    }
    
}