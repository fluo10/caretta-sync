use rusqlite::{Error, Connection};

pub fn migrate(con: &mut Connection) -> Result<(), Error>{
    let tx = con.transaction()?;
    tx.execute_batch(
        "BEGIN;
            CREATE TABLE peer (
                id         INTEGER PRIMARY KEY,
                local_id   INTEGER NOT NULL UNIQUE,
                public_key BLOB UNIQUE NOT NULL,
            );
            CREATE TABLE received_authorization_request (
                id           INTEGER PRIMARY KEY,
                request_id   INTEGER NOT NULL UNIQUE,
                public_key   BLOB NOT NULL UNIQUE,
                node_info    TEXT,
                created_at   TEXT NOT NULL,
                responded_at TEXT
            );
            CREATE TABLE sent_authorization_request (
                id           INTEGER PRIMARY KEY,
                request_id   INTEGER NOT NULL UNIQUE,
                public_key.  BLOB NOT NULL UNIQUE,
                passcode     TEXT NOT NULL,
                created_at   TEXT NOT NULL,
                sent_at      TEXT
            );
            CREATE TABLE authorized_peer (
                id                       INTEGER PRIMARY KEY,
                node_id                  BLOB NOT NULL UNIQUE,
                last_synced_at           TEXT,
                last_sent_version_vector BLOB
                created_at               TEXT NOT NULL,
                updated_at               TEXT NOT NULL,
            );
            CREATE TABLE authorization (
                id         INTEGER PRIMARY KEY,
                node_id    BLOB UNIQUE NOT NULL,
                passcode   TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
            );
            COMMIT;",
    )?;
    tx.pragma_update(None,  "user_version", 1)?;
    tx.commit()?;
    Ok(())
}