pub mod error;
mod log;
mod p2p;
mod rpc;
mod storage;
mod parsed;
pub use parsed::ParsedConfig;

use crate::utils::{emptiable::Emptiable, mergeable::Mergeable};
pub use error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub use p2p::{P2pConfig, PartialP2pConfig};
pub use rpc::*;
pub use storage::{PartialStorageConfig, StorageConfig};
pub use log::{LogConfig, PartialLogConfig, LogLevel, LogLevelParseError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "cli")]
use clap::Args;
