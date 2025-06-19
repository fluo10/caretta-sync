use crate::{config::StorageConfig, error::Error, global::SimpleGlobal};
use tokio::sync::OnceCell;
use uuid::fmt::Simple;

pub static STORAGE_CONFIG: SimpleGlobal<StorageConfig> = SimpleGlobal::const_new();

