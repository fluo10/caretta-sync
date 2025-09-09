use std::{fs::create_dir_all, path::{Path, PathBuf}, sync::OnceLock};

use rusqlite::Connection;

use crate::{data::local::migration::migrate, error::Error};

pub static LOCAL_DATABASE_CONNECTION: GlobalLocalDatabaseConnection = GlobalLocalDatabaseConnection::const_new();

pub struct GlobalLocalDatabaseConnection {
    path: OnceLock<PathBuf>
}

fn path_to_connection_or_panic<P>(path: &P) -> Connection 
where  
    P: AsRef<Path>
{
    Connection::open(path.as_ref()).expect("Failed to open database connection for local data")
}

impl GlobalLocalDatabaseConnection {
    const fn const_new() ->  Self {
        Self {
            path: OnceLock::new()
        }
    }

    pub fn get_or_init<P>(&self, path: &P) -> Connection
    where 
        P: AsRef<Path>,
    {
        path_to_connection_or_panic(self.path.get_or_init(|| {
            let path = path.as_ref();
            let parent = path.parent().expect("Database path should have parent directory");
            create_dir_all(parent).expect("Failed to create parent directory of database");
            let mut conn = path_to_connection_or_panic(&path);
            migrate(&mut conn).expect("Local database migration should be done correctly");
            path.to_path_buf()
        }))
    }
        
    pub fn get(&self) -> Option<Connection> {
        self.path.get().map(|path| {
            path_to_connection_or_panic(path)
        })
    }
    pub fn get_unchecked(&self) -> Connection {
        self.get().expect("Global database for local data mulst be initialized before use")
    }
}