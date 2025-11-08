//! Provides configs parsed from file, command-line args and environment valiables.
mod error;
mod log;
mod p2p;
mod rpc;
mod storage;
pub mod types;
use clap::Args;
pub use error::ParsedConfigError;
pub use log::ParsedLogConfig;
pub use p2p::ParsedP2pConfig;
pub use rpc::ParsedRpcConfig;
pub use storage::ParsedStorageConfig;

use sea_orm_migration::MigratorTrait;
use serde::{Deserialize, Serialize, ser::Error};

#[cfg(feature="backend")]
use crate::config::{P2pConfig, StorageConfig};
use crate::{
    config::{LogConfig, RpcConfig},
    utils::{emptiable::Emptiable, mergeable::Mergeable},
};
use std::{
    fmt::Display,
    fs::File,
    io::Read,
    marker::PhantomData,
    path::{Path, PathBuf},
};

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct ParsedConfig {
    #[command(flatten)]
    pub storage: Option<ParsedStorageConfig>,
    #[command(flatten)]
    pub rpc: Option<ParsedRpcConfig>,
    #[command(flatten)]
    pub p2p: Option<ParsedP2pConfig>,
    #[command(flatten)]
    pub log: Option<ParsedLogConfig>,
}

/// A partial config parsed from config file, cli args, etc.
impl ParsedConfig {
    fn default(app_name: &'static str) -> Self {
        Self {
            storage: Some(ParsedStorageConfig::default(app_name)),
            rpc: Some(ParsedRpcConfig::default(app_name)),
            p2p: None,
            log: Some(ParsedLogConfig::default()),
        }
    }

    /// Fill empty configuration fields with default values and return.
    pub fn with_default(self, app_name: &'static str) -> Self {
        let mut result = Self::default(app_name);
        result.merge(self);
        result
    }

    /// Build [`StorageConfig`] from own [`ParsedStorageConfig`]
    #[cfg(feature="backend")]
    pub fn to_storage_config(&self) -> Result<StorageConfig, ParsedConfigError> {
        self.storage
            .as_ref()
            .ok_or(ParsedConfigError::MissingConfig("storage.*"))?
            .clone()
            .try_into()
    }

    /// Build [`P2pConfig`] from own [`ParsedP2pConfig`]
    #[cfg(feature="backend")]
    pub fn to_p2p_config(&self) -> Result<P2pConfig, ParsedConfigError> {
        self.p2p
            .as_ref()
            .ok_or(ParsedConfigError::MissingConfig("P2P.*"))?
            .clone()
            .try_into()
    }

    /// Build [`RpcConfig`] from own [`ParsedRpcConfig`]
    pub fn to_rpc_config(&self) -> Result<RpcConfig, ParsedConfigError> {
        self.rpc
            .as_ref()
            .ok_or(ParsedConfigError::MissingConfig("rpc.*"))?
            .clone()
            .try_into()
    }
    /// Build [`LogConfig`] from own [`ParsedLogConfig`]
    pub fn to_log_config(&self) -> Result<LogConfig, ParsedConfigError> {
        self.log
            .as_ref()
            .ok_or(ParsedConfigError::MissingConfig("log.*"))?
            .clone()
            .try_into()
    }
    /// Read or create target config file
    pub fn read_or_create_from_path<T>(path: T) -> Result<Self, ParsedConfigError>
    where
        T: AsRef<Path>,
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path)?;
        }
        let mut file = File::open(path.as_ref())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get default config path
    pub fn default_config_path(app_name: &'static str) -> Result<PathBuf, ParsedConfigError> {
        const DEFAULT_FILE_NAME: &str = "config.toml";
        let mut path = dirs::config_local_dir().ok_or(ParsedConfigError::ConfigDir)?;
        path.push(app_name);
        path.push(DEFAULT_FILE_NAME);
        Ok(path)
    }

    /// Read or create target config file at the default config path
    pub fn read_or_create(app_name: &'static str) -> Result<Self, ParsedConfigError> {
        let config_dir = Self::default_config_path(app_name)?;
        Self::read_or_create_from_path(config_dir)
    }
    pub fn init_tracing_subscriber(&self) {
        self.to_log_config().unwrap().init_tracing_subscriber();
    }
    #[cfg(feature="server")]
    pub async fn into_server_context<M>(
        app_name: &'static str,
        migrator: PhantomData<M>,
    ) -> Result<ServerContext, Error>
    where
        M: MigratorTrait,
    {
        let config = config.as_ref();
        let rpc_config = config.to_rpc_config()?;
        let p2p_config = config.to_p2p_config()?;
        let storage_config = config.to_storage_config()?;
        let database_connection = storage_config.to_database_connection(migrator).await;
        let iroh_router = p2p_config.to_iroh_router(app_name).await?;
        Ok(Self {
            app_name,
            storage_config,
            database_connection,
            iroh_router,
        })
    }
    #[cfg(feature="client")]
    pub fn into_client_context<T>(app_name: &'static str, config: T) -> Result<Self, ConfigError>
    where
        T: AsRef<ParsedConfig>,
    {
        let config = config.as_ref();
        let rpc_config = config.to_rpc_config()?;
        Ok(Self {
            app_name,
            rpc_config,
        })
    }
}

impl AsRef<ParsedConfig> for ParsedConfig {
    fn as_ref(&self) -> &ParsedConfig {
        self
    }
}

impl Emptiable for ParsedConfig {
    fn empty() -> Self {
        Self {
            p2p: None,
            storage: None,
            rpc: None,
            log: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.rpc.is_empty() && self.storage.is_empty() && self.log.is_empty()
    }
}

impl Mergeable for ParsedConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.rpc.merge(other.rpc);
        self.storage.merge(other.storage);
        self.log.merge(other.log);
    }
}

impl Display for ParsedConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}
