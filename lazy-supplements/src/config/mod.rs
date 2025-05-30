mod node;
mod server;

use std::path::Path;
use crate::error::Error;
pub use node::NodeConfig;
use serde::{Deserialize, Serialize};
pub use server::{
    PartialServerConfig,
    ServerConfig,
    DEFAULT_LISTEN_IPS,
    DEFAULT_PORT,
    DEFAULT_PARTIAL_SERVER_CONFIG,
    DEFAULT_SERVER_CONFIG
};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConfig {
    node: Option<NodeConfig>,
    server: Option<ServerConfig>,
}

impl PartialConfig {
    pub fn new() -> Self {
        PartialConfig {
            node: Some(NodeConfig::new()),
            server: None,
        }
    }
    pub async fn read_or_create<T>(path: T) -> Result<Self, Error> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path).await?;
        }
        Self::read_from(&path).await
    }
    pub async fn read_from<T>(path:T) -> Result<Self, Error> 
    where 
    T: AsRef<Path>
    {
        let mut file = File::open(path.as_ref()).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: PartialConfig = toml::from_str(&content)?;
        Ok(config)
    }
    pub async fn write_to<T>(&self, path:T) -> Result<(), Error> 
    where 
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            let _ = File::create(&path).await?;
        }
        let mut file = File::open(&path).await?;
        file.write_all(toml::to_string(self)?.as_bytes()).await?;
        Ok(())
    }
}



