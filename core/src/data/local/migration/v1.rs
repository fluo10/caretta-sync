use rusqlite::{Connection, Error, Transaction};

pub fn migrate(tx: &Transaction) -> Result<(), Error>{
    tx.execute_batch(
        "CREATE TABLE remote_node (
                id         INTEGER PRIMARY KEY,
                public_id  INTEGER NOT NULL UNIQUE,
                public_key BLOB UNIQUE NOT NULL
            );
            CREATE TABLE authorization_request (
                id             INTEGER PRIMARY KEY,
                uuid           BLOB NOT NULL UNIQUE,
                public_id      INTEGER NOT NULL UNIQUE,
                remote_node_id INTEGER NOT NULL UNIQUE,
                created_at     TEXT NOT NULL,
                closed_at      TEXT,
                FOREIGN KEY(remote_node_id) REFERENCES remote_node(id)
            );
            CREATE TABLE received_authorization_request (
                id                       INTEGER PRIMARY KEY,
                authorization_request_id INTEGER NOT NULL UNIQUE,
                node_note         TEXT,
                FOREIGN KEY(authorization_request_id) REFERENCES authorization_request(id)
            );
            CREATE TABLE sent_authorization_request (
                id                       INTEGER PRIMARY KEY,
                authorization_request_id INTEGER NOT NULL UNIQUE,
                passcode                 TEXT NOT NULL,
                FOREIGN KEY(authorization_request_id) REFERENCES authorization_request(id)
            );
            CREATE TABLE authorized_remote_node (
                id                       INTEGER PRIMARY KEY,
                uuid                     BLOB UNIQUE NOT NULL,
                public_id                INTEGER NOT NULL UNIQUE,
                public_key               BLOB NOT NULL UNIQUE,
                note                     TEXT NOT NULL,
                last_synced_at           TEXT,
                last_sent_version_vector BLOB,
                created_at               TEXT NOT NULL,
                updated_at               TEXT NOT NULL
            );",
    )?;

    Ok(())
}