pub mod error;
mod core;

use std::path::Path;
use crate::error::Error;
pub use core::{ CoreConfig, PartialCoreConfig };
pub use error::ConfigError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};

pub trait PartialConfig<T>: From<T>
where T: TryFrom<Self> {
    fn default() -> Self;
    fn empty() -> Self;
    fn merge(&mut self, other: Self);
}

pub trait ConfigFile: DeserializeOwned + Serialize {
    fn new() -> Self;

    async fn read_or_create<T>(path: T) -> Result<Self, Error> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path).await?;
        }
        Self::read_from(&path).await
    }
    async fn read_from<T>(path:T) -> Result<Self, Error> 
    where 
    T: AsRef<Path>
    {
        let mut file = File::open(path.as_ref()).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    async fn write_to<T>(&self, path:T) -> Result<(), Error> 
    where 
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path).await?;
        }
        let mut file = File::create(&path).await?;
        file.write_all(toml::to_string(self)?.as_bytes()).await?;
        Ok(())
    }
}