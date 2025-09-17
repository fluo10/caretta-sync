//! Structs about cached peer.

use std::os::unix::raw::time_t;

use caretta_id::DoubleId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::{params, types::FromSqlError, Connection};
use uuid::Uuid;

use crate::{data::local::{self, LocalRecord}, global::LOCAL_DATABASE_CONNECTION};

/// Peer information cached in local database.
/// 
/// - Currently this only contain local id and public key (=node id) of iroh.
/// - This is a junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual peer information is managed by iroh endpoint and not contained in this model.
/// - Once a peer is authorized, it is assigned a global (=synced) ID as authorized_peer so essentially this local id targets unauthorized peers.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct PeerRecord {

    /// local id of peer.
    /// this id is use only the node itself and not synced so another node has different local_peer_id even if its public_key is same.
    pub local_peer_id: DoubleId,
    pub public_key: PublicKey,
}

impl PeerRecord {
    pub fn get_or_insert_by_public_key(public_key: &PublicKey) -> Result<Self, rusqlite::Error> {
        match Self::get_by_public_key(public_key)? {
            Some(x) => Ok(x),
            None => {
                let new = Self{
                    local_peer_id: rand::random(),
                    public_key: public_key.clone(),
                };
                new.insert()?;
                Ok(new)
            }
        }

    }
    pub fn get_by_local_id(local_id: &DoubleId) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_where("WHERE local_peer_id = ?1", (local_id,))
    }
    pub fn get_by_public_key(public_key: &PublicKey) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_where("WHERE public_Key = ?1", (public_key.as_bytes(),))
    }
}

impl LocalRecord for PeerRecord {
    const TABLE_NAME: &str = "peer";
    const DEFAULT_COLUMNS: &[&str] = &[
        "local_peer_id", 
        "public_key"
    ];
    type DefaultParams<'a> = (&'a DoubleId, &'a [u8;32]);
    fn as_default_params<'a>(&'a self) -> Self::DefaultParams<'a>
    {
        (&self.local_peer_id, &self.public_key.as_bytes())
    }

    fn from_default_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            local_peer_id: row.get(0)?,
            public_key: PublicKey::from_bytes(&row.get(1)?).map_err(|e| FromSqlError::Other(Box::new(e)))?
        })
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

#[cfg(test)]
mod tests {
    use iroh::SecretKey;

    use crate::tests::TEST_CONFIG;

    use super::*;

    #[test]
    fn insert_get_peer_record() {
        LOCAL_DATABASE_CONNECTION.get_or_init(&TEST_CONFIG.storage.get_local_database_path());
        let key = SecretKey::generate(&mut rand::rngs::OsRng);
        let pubkey = key.public();
        let record = PeerRecord::get_or_insert_by_public_key(&pubkey).unwrap();
        assert_eq!(record, PeerRecord::get_by_local_id(&record.local_peer_id).unwrap().unwrap());
        assert_eq!(record, PeerRecord::get_by_public_key(&record.public_key).unwrap().unwrap());

    }
}