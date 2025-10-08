use caretta_id::{DtidId, StidId};
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};

use crate::{data::local::LocalRecord, global::LOCAL_DATABASE_CONNECTION};

/// Response of node authentication.
#[derive(Debug, Clone)]
pub struct ReceivedAuthorizationRequestRecord {
    id: u32,
    authorization_request_id: u32,
    peer_note: String,
}

impl LocalRecord for ReceivedAuthorizationRequestRecord {
    const TABLE_NAME: &str = "received_authorization_request";
    
    const SELECT_COLUMNS: &[&str] = &[
        "id",
        "authorization_request_id",
        "peer_note"
    ];
    
    const INSERT_COLUMNS: &[&str] = &[
        "authorization_request_id",
        "peer_note"
    ];
    
    type InsertParams<'a> = (&'a u32, &'a str);
    
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            authorization_request_id: row.get(1)?,
            peer_note: row.get(2)?
        })
    }
    
    

}