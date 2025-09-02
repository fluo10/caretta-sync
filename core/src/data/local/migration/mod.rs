mod v1;

use rusqlite::{ffi::Error, Connection};
use tracing::{event, Level};

pub fn migrate(con: &Connection) -> Result<(), Error>{
    let version: u32 = con.pragma_query_value(None,"user_version", |row| row.get(0)).expect("Failed to get user_version");
    if version < 1 {
        event!(Level::INFO, "Migrate local db to version 1");
        v1::migrate(con)?;
        event!(Level::INFO, "Migration done.");
    } 
    Ok(())
}