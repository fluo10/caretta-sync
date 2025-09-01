use rusqlite::{ffi::Error, Connection};
use tracing::{event, Level};


pub fn migrate(con: &mut Connection) -> Result<(), Error>{
    let version: u32 = con.pragma_query_value(None,"user_version", |row| row.get(0))?;
    if version < 1 {
        event!(Level::INFO, "Migrate local db to version 1");
        let tx = con.transaction()?;
        tx.execute(
            "CREATE TABLE known_peer (
                id         INTEGER PRIMARY KEY,
                peer_id    TEXT UNIQUE NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
            )",
            ()
        )?;
        tx.execute(
            "CREATE TABLE known_address (
                id            INTEGER PRIMARY KEY,
                known_peer_id INTEGER NOT NULL, 
                multiaddr     TEXT UNIQUE NOT NULL,
                created_at    TEXT NOT NULL,
                updated_at    TEXT NOT NULL,
                protocol      TEXT NOT NULL,
                FOREIGN KEY(knwon_peer_id) REFERENCES knwon_peer(id),
            )",
            ()
        )?;
        tx.execute(
            "CREATE TABLE authorized_peer (
            id            INTEGER PRIMARY KEY,
            known_peer_id INTEGER NOT NULL UNIQUE,
            synced_at     TEXT,
            created_at    TEXT NOT NULL,
            updated_at    TEXT NOT NULL,
            FOREIGN KEY(known_peer_id) REFERENCES knwon_peer(id)",
            ()
        )?;
        tx.pragma_update(None,  "user_version", 1)?;
        tx.commit()?;
        event!(Level::INFO, "Migration done.")
    }
}