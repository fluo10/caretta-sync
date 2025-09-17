use caretta_id::SingleId;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use rusqlite::types::FromSqlError;

use crate::{data::local::LocalRecord, global::LOCAL_DATABASE_CONNECTION};

/// Request of node authentication.
#[derive(Debug, Clone)]
pub struct SentAuthorizationRequest {
    request_id: SingleId,
    public_key: PublicKey,
    passcode: String,
    created_at: DateTime<Local>,
    responded_at: Option<DateTime<Local>>,
}

impl LocalRecord for SentAuthorizationRequest {

    const TABLE_NAME: &str = "sent_authorization";
    const DEFAULT_COLUMNS: &[&str] = &[
        "request_id",
        "public_key",
        "passcode",
        "created_at",
        "responded_at"
    ];
    type DefaultParams<'a> = (&'a SingleId, &'a [u8;32], &'a str, NaiveDateTime, Option<NaiveDateTime>)
        where 
            Self: 'a;
    fn from_default_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        let created_at: NaiveDateTime = row.get(2)?;
        let responded_at: Option<NaiveDateTime> = row.get(3)?;
        Ok(Self {
            request_id: row.get(0)?,
            public_key: PublicKey::from_bytes(&row.get(1)?).map_err(|e| FromSqlError::Other(Box::new(e)))?,
            passcode: row.get(2)?,
            created_at: DateTime::from(created_at.and_utc()),
            responded_at: responded_at.map(|x| DateTime::from(x.and_utc())),
        })
    }
    fn insert(&self) -> Result<(), rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();
        connection.execute(
            &(String::from("INSERT INTO ") + Self::TABLE_NAME + " (" + &Self::DEFAULT_COLUMNS.join(", ") + ") VALUES (?1, ?2, ?3, ?4, ?5)"),
            (
                &self.request_id,
                &self.public_key.as_bytes(),
                &self.passcode,
                &self.created_at.naive_utc(),
                &self.responded_at.map(|x| x.naive_utc())
            ),
        )?;
        Ok(())
    }
    fn get_all() -> Result<Vec<Self>, rusqlite::Error> {
        let connection = LOCAL_DATABASE_CONNECTION.get_unchecked();

        let mut stmt = connection.prepare(&(String::from("SELECT ") + &Self::DEFAULT_COLUMNS.join(", ") + " FROM " + Self::TABLE_NAME))?;
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
    
    fn as_default_params<'a>(&'a self) -> Self::DefaultParams<'a> {
        (&self.request_id, &self.public_key.as_bytes(), &self.passcode, self.created_at.naive_utc(), self.responded_at.map(|x| x.naive_utc()))
    }
}