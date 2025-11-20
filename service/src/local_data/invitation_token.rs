use core::time;
use std::convert::Infallible;

use caretta_id::CarettaId;
use caretta_sync_core::context::{ServiceContext, ServiceContextExt};
use chrono::{DateTime, Duration, DurationRound, Local, SubsecRound};
use redb::{ReadableDatabase, TableDefinition, TypeName, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::TokenStatus;

const LOCAL_INVITAION_TOKEN_TABLE: TableDefinition<CarettaId, LocalInvitationToken> = TableDefinition::new("invitation_token");



/// Inviation token data stored in local database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalInvitationToken {
    pub id: CarettaId,
    pub created_at: DateTime<Local>,

    /// The timestamp when the token expires
    ///
    /// Since the token does not contain subseconds, this timestamp is also rounded to the newarest second.
    pub expires_at: DateTime<Local>,
    pub closed_at: Option<DateTime<Local>>,
    pub status: TokenStatus,
}

impl LocalInvitationToken {

    /// Returns new, unused id.
    async fn gen_new_id<T>(context: &T) -> Result<CarettaId, redb::Error> 
    where 
    T: ServiceContextExt
    {
        let mut id = CarettaId::random();
        let db = context.as_local_database();
        let read_txn = db.begin_read()?;
        match read_txn.open_table(LOCAL_INVITAION_TOKEN_TABLE) {
            Ok(table) => { 
                while table.get(id)?.is_some() {
                    id = CarettaId::random();
                }
                Ok(id)
            },
            Err(redb::TableError::TableDoesNotExist(_)) => Ok(id),
            Err(e) => Err(e.into())
        }
    }

    /// Create new `LocalInvitationToken` record.
    pub async fn new<T>(context: &T, duration: Duration) -> Result<Self, redb::Error> 
    where 
        T: ServiceContextExt
    {
        let timestamp = Local::now().round_subsecs(3);
        let id = Self::gen_new_id(context).await?;
        let db = context.as_local_database();
        let write_txn = db.begin_write()?;
        let result = Self {
            id:id,
            created_at: timestamp.clone(),
            expires_at: (timestamp + duration),
            status: TokenStatus::Pending,
            closed_at: None,
        };
        {
            let mut table = write_txn.open_table(LOCAL_INVITAION_TOKEN_TABLE)?;
            let _ = table.insert(&id, &result)?;  
        }
        Ok(result)
    }
}

impl Value for LocalInvitationToken {
    type SelfType<'a> = Self;

    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a {
            let mut buf = Vec::new();
        ciborium::from_reader_with_buffer(data, &mut buf).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b {
        let mut buf = Vec::new();
        ciborium::into_writer(value, &mut buf).unwrap();
        buf
    }

    fn type_name() -> redb::TypeName {
        TypeName::new(stringify!(LocalInvitationToken))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iroh::{PublicKey, SecretKey};
    use rand::Rng;

    #[tokio::test]
    async fn insert() {
        let context = crate::tests::service_conext().await;
        
        let token = LocalInvitationToken::new(context, Duration::seconds(1)).await.unwrap();

    }
}
