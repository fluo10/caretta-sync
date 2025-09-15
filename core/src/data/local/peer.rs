//! Structs about cached peer.

use std::os::unix::raw::time_t;

use caretta_id::DoubleId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::{params, types::FromSqlError, Connection};
use uuid::Uuid;

use crate::{data::local::LocalModel, global::LOCAL_DATABASE_CONNECTION};

/// Peer information cached in local database.
/// 
/// - Currently this only contain local id and public key (=node id) of iroh.
/// - This is a junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual peer information is managed by iroh endpoint and not contained in this model.
/// - Once a peer is authorized, it is assigned a global (=synced) ID as authorized_peer so essentially this local id targets unauthorized peers.
/// 
pub struct Peer {
    pub local_id: DoubleId,
    pub public_key: PublicKey,
}

impl Peer {
    pub fn get_by_local_id(local_id: DoubleId) -> Result<Option<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(Some(connection.query_row(
            &("SELECT ".to_string() + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME + " WHERE local_id=(?1)"),
            params![local_id],
            Self::from_default_row
        )?))
    }
    pub fn get_by_public_key(public_key: PublicKey) -> Result<Option<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        Ok(Some(connection.query_row(
            &("SELECT ".to_string() + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME + " WHERE public_key=(?1)"),
            params![public_key.as_bytes()],
            Self::from_default_row
        )?))
    }
}

impl LocalModel for Peer {
    const TABLE_NAME: &str = "peer";
    const DEFAULT_COLUMNS: &[&str] = &[
        "local_id", 
        "public_key"
    ];

    fn from_default_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {

        Ok(Self {
            local_id: row.get(0)?,
            public_key: PublicKey::from_bytes(&row.get(1)?).map_err(|e| FromSqlError::Other(Box::new(e)))?
        })
    }
    fn insert(&self) -> Result<(), rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        
        connection.execute(
            &("INSERT INTO ".to_owned() + Self::TABLE_NAME + " (" + &Self::DEFAULT_COLUMNS.join(", ") + ") VALUES (?1, ?2, ?3, ?4)"),
            (&self.local_id, &self.public_key.as_bytes()),
        )?;
        Ok(())
    }
    fn get_all() -> Result<Vec<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        let mut stmt = connection.prepare(&("SELECT ".to_string() + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME))?;
        let rows = stmt.query_map(
            [],
            Self::from_default_row
        )?;
        let mut result= Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}