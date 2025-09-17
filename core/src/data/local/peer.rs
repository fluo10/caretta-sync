//! Structs about cached peer.

use std::os::unix::raw::time_t;

use caretta_id::DoubleId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::{params, types::{FromSqlError, Null}, Connection};
use uuid::Uuid;

use crate::{data::local::{self, LocalRecord}, global::LOCAL_DATABASE_CONNECTION};

/// Peer information cached in local database.
/// 
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - This is a junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual peer information is managed by iroh endpoint and not contained in this model.
/// - Once a peer is authorized, it is assigned a global (=synced) ID as authorized_peer so essentially this local id targets unauthorized peers.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct PeerRecord {

    /// primary key.
    pub id: u32,

    /// uid of peer.
    /// this id is use only the node itself and not synced so another node has different local_peer_id even if its public_key is same.
    pub uid: DoubleId,
    pub public_key: PublicKey,
}

impl PeerRecord {
    pub fn get_or_insert_by_public_key(public_key: &PublicKey) -> Result<Self, rusqlite::Error> {
        match Self::get_by_public_key(public_key)? {
            Some(x) => Ok(x),
            None => {
                let new_uid: DoubleId = rand::random();
                Ok(Self::insert((&new_uid, public_key.as_bytes()))?)
            }
        }

    }
    pub fn get_by_uid(local_id: &DoubleId) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_where("WHERE local_peer_id = ?1", (local_id,))
    }
    pub fn get_by_public_key(public_key: &PublicKey) -> Result<Option<Self>, rusqlite::Error> {
        Self::get_one_where("WHERE public_Key = ?1", (public_key.as_bytes(),))
    }
}

impl LocalRecord for PeerRecord {
    const TABLE_NAME: &str = "peer";
    const SELECT_COLUMNS: &[&str] = &[
        "id",
        "uid", 
        "public_key"
    ];
    const INSERT_COLUMNS: &[&str] = &[
        "uid",
        "public_key"
    ];
    type InsertParams<'a> = (&'a DoubleId, &'a [u8;32]);

    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            uid: row.get(1)?,
            public_key: PublicKey::from_bytes(&row.get(2)?).map_err(|e| FromSqlError::Other(Box::new(e)))?
        })
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
        assert_eq!(record, PeerRecord::get_by_uid(&record.uid).unwrap().unwrap());
        assert_eq!(record, PeerRecord::get_by_public_key(&record.public_key).unwrap().unwrap());
    }
}