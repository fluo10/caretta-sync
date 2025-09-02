use rusqlite::{Error, Connection};

pub fn migrate(con: &mut Connection) -> Result<(), Error>{
    let tx = con.transaction()?;
    tx.execute_batch(
        "BEGIN;
            CREATE TABLE peer (
                id             INTEGER PRIMARY KEY,
                libp2p_peer_id TEXT UNIQUE NOT NULL,
                created_at     TEXT NOT NULL,
                updated_at     TEXT NOT NULL,
            );
            CREATE INDEX idx_peer_created_at ON peer(created_at);
            CREATE INDEX idx_peer_updated_at ON peer(updated_at);
            CREATE TABLE address (
                id            INTEGER PRIMARY KEY,
                peer_id       INTEGER NOT NULL, 
                multiaddr     TEXT UNIQUE NOT NULL,
                created_at    TEXT NOT NULL,
                updated_at    TEXT NOT NULL,
                protocol      TEXT NOT NULL,
                FOREIGN KEY(peer_id) REFERENCES peer(id)
            );
            CREATE INDEX idx_address_created_at ON address(created_at);
            CREATE INDEX idx_address_updated_at ON address(updated_at);
            CREATE TABLE authorized_peer (
                id            INTEGER PRIMARY KEY,
                peer_id       INTEGER NOT NULL UNIQUE,
                synced_at     TEXT,
                created_at    TEXT NOT NULL,
                updated_at    TEXT NOT NULL,
                FOREIGN KEY(peer_id) REFERENCES peer(id)
            )",
    )?;
    tx.pragma_update(None,  "user_version", 1)?;
    tx.commit()?;
    Ok(())
}