mod node;

use std::path::Path;
use crate::error::Error;
pub use node::{ NodeConfig, RawNodeConfig };
use serde::{Deserialize, Serialize};
pub use crate::global::{
    DEFAULT_LISTEN_IPS,
    DEFAULT_PORT,
};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConfig {
    node: Option<NodeConfig>,
}


