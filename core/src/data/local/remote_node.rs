//! Structs about cached remote_node.

use std::os::unix::raw::time_t;

use tripod_id::Double;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::{params, types::{FromSqlError, Null}, Connection};
use uuid::Uuid;

use crate::{data::local::{self, InsertableLocalRecord, LocalRecord, LocalRecordId, NoLocalRecordId, SelectableLocalRecord}, global::LOCAL_DATABASE_CONNECTION};

/// RemoteNode information cached in local database.
/// 
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - This is a junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual remote_node information is managed by iroh endpoint and not contained in this model.
/// - Once a remote_node is authorized, it is assigned a global (=synced) ID as authorized_remote_node so essentially this local id targets unauthorized remote_nodes.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct RemoteNodeRecord<T> {

    /// serial primary key.
    pub id: T,

    /// public tripod id of remote_node.
    /// this id is use only the node itself and not synced so another node has different local_remote_node_id even if its public_key is same.
    pub public_id: Double,

    /// Iroh public key
    pub public_key: PublicKey,
}

impl RemoteNodeRecord<LocalRecordId> {
    pub fn get_or_insert_by_public_key(public_key: &PublicKey) -> Result<Self, rusqlite::Error> {
        match Self::get_by_public_key(public_key) {
            Ok(x) => Ok(x),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let new = RemoteNodeRecord{
                    id: NoLocalRecordId{},
                    public_id: rand::random(),
                    public_key: public_key.clone()                    
                };
                Ok(new.insert()?)
            },
            Err(e) => Err(e)
        }

    }
    pub fn get_by_public_id(public_id: &Double) -> Result<Self, rusqlite::Error> {
        Self::get_one_where("WHERE public_id = ?1", (public_id,))
    }
    pub fn get_by_public_key(public_key: &PublicKey) -> Result<Self, rusqlite::Error> {
        Self::get_one_where("WHERE public_Key = ?1", (public_key.as_bytes(),))
    }
}

impl<T> LocalRecord for RemoteNodeRecord<T> {
    const TABLE_NAME: &str = "remote_node";
    const COLUMNS: &[&str] = &[
        "id",
        "public_id", 
        "public_key"
    ];

    type RowValues = (T, Double, [u8;32]);
}

impl SelectableLocalRecord for RemoteNodeRecord<LocalRecordId> {
    fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            public_id: row.get(1)?,
            public_key: PublicKey::from_bytes(&row.get(2)?).map_err(|e| FromSqlError::Other(Box::new(e)))?
        })
    }
}

impl TryFrom<(LocalRecordId, Double, [u8;32])> for RemoteNodeRecord<LocalRecordId> {
    type Error = rusqlite::Error;
    fn try_from(value: (LocalRecordId, Double, [u8;32])) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.0,
            public_id: value.1,
            public_key: PublicKey::from_bytes(&value.2).map_err(|x| FromSqlError::Other(Box::new(x)))?
        })
    }
}

impl InsertableLocalRecord for RemoteNodeRecord<NoLocalRecordId> {    
    type LocalRecord = RemoteNodeRecord<LocalRecordId>;
    
}

impl From<RemoteNodeRecord<NoLocalRecordId>> for (NoLocalRecordId, Double, [u8;32]){
    fn from(value: RemoteNodeRecord<NoLocalRecordId>) -> Self {
        (value.id, value.public_id, value.public_key.as_bytes().to_owned())
    }


}

#[cfg(test)]
mod tests {
    use iroh::SecretKey;

    use crate::tests::TEST_CONFIG;

    use super::*;

    #[test]
    fn insert_get_remote_node_record() {
        LOCAL_DATABASE_CONNECTION.get_or_init(&TEST_CONFIG.storage.get_local_database_path());
        let key = SecretKey::generate(&mut rand::rngs::OsRng);
        let pubkey = key.public();
        let record = RemoteNodeRecord::get_or_insert_by_public_key(&pubkey).unwrap();
        assert_eq!(record, RemoteNodeRecord::get_by_public_id(&record.public_id).unwrap());
        assert_eq!(record, RemoteNodeRecord::get_by_public_key(&record.public_key).unwrap());
    }
}