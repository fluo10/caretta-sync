use std::{any::type_name, collections::HashMap, net::{IpAddr, Ipv4Addr}, path::{Path, PathBuf}, sync::LazyLock};

use crate::{config::{StorageConfig}, error::Error }; 
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};

mod config;
pub use config::*;
use uuid::{ContextV7, Timestamp, Uuid};

pub fn generate_uuid() -> Uuid {
    Uuid::new_v7(Timestamp::now(ContextV7::new()))
}

fn uninitialized_message<T>(var: T) -> String {
    format!("{} is uninitialized!", &stringify!(var))
}
