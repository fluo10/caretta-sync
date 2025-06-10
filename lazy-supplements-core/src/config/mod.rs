mod node;

use std::path::Path;
use crate::error::Error;
pub use node::{ NodeConfig, RawNodeConfig };
use serde::{Deserialize, Serialize};

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConfig {
    node: Option<NodeConfig>,
}


