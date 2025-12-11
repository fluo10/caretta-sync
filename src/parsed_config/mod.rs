//! Provides configs parsed from file, command-line args and environment valiables.
mod error;
mod log;
mod p2p;
mod ipc;
mod storage;
pub mod types;
#[cfg(feature = "client")]
use caretta_sync_core::context::ClientContext;
#[cfg(feature = "server")]
use caretta_sync_core::{
    config::{P2pConfig, StorageConfig},
    context::ServerContext,
};
use clap::Args;
pub use error::ParsedConfigError;
pub use log::ParsedLogConfig;
pub use p2p::ParsedP2pConfig;
pub use ipc::ParsedIpcConfig;
#[cfg(feature = "server")]
use redb::Database;
pub use storage::ParsedStorageConfig;
use serde::{Deserialize, Serialize, ser::Error};

use caretta_sync_core::{
    config::{LogConfig, IpcConfig},
    util::{Emptiable, Mergeable},
};
use std::{
    fmt::Display,
    fs::File,
    io::Read,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::types::Verbosity;

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ParsedConfig {
    #[command(flatten)]
    #[serde(default, skip_serializing_if  = "ParsedStorageConfig::is_empty")]
    pub storage: ParsedStorageConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if  = "ParsedIpcConfig::is_empty")]
    pub ipc: ParsedIpcConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if  = "ParsedP2pConfig::is_empty")]
    pub p2p: ParsedP2pConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if  = "ParsedLogConfig::is_empty")]
    pub log: ParsedLogConfig,
}

/// A partial config parsed from config file, cli args, etc.
impl ParsedConfig {
    fn default(app_name: &'static str) -> Self {
        Self {
            storage: ParsedStorageConfig::default(app_name),
            ipc: ParsedIpcConfig::default(app_name),
            p2p: ParsedP2pConfig::empty(),
            log: ParsedLogConfig::default(),
        }
    }

    /// Fill empty configuration fields with default values and return.
    pub fn with_default(self, app_name: &'static str) -> Self {
        let mut result = Self::default(app_name);
        result.merge(self);
        result
    }

    /// Fill empty configuration fields with database values
    #[cfg(feature = "server")]
    pub fn with_local_database(mut self) -> Self {
        use caretta_sync_service::local_data::LocalP2pConfigExt;
        let db = self.to_storage_config().unwrap().to_local_database();
        let p2p_config = P2pConfig::get_or_init_db(&db);
        self.merge(ParsedP2pConfig::from(p2p_config));
        self
    }

    /// Remove server-only configurations
    #[cfg(feature = "client")]
    pub fn except_server_only_config(mut self) -> Self {
        self.p2p = ParsedP2pConfig::empty();
        self.storage = ParsedStorageConfig::empty();
        self
    }

    /// Build [`StorageConfig`] from own [`ParsedStorageConfig`]
    #[cfg(feature = "server")]
    pub fn to_storage_config(&self) -> Result<StorageConfig, ParsedConfigError> {
        self.storage.clone().try_into()
    }

    /// Build [`P2pConfig`] from own [`ParsedP2pConfig`]
    #[cfg(feature = "server")]
    pub fn to_p2p_config(&self) -> Result<P2pConfig, ParsedConfigError> {
        self.p2p.clone().try_into()
    }

    /// Build [`IpcConfig`] from own [`ParsedIpcConfig`]
    pub fn to_ipc_config(&self) -> Result<IpcConfig, ParsedConfigError> {
        self.ipc.clone().try_into()
    }
    /// Build [`LogConfig`] from own [`ParsedLogConfig`]
    pub fn to_log_config(&self) -> Result<LogConfig, ParsedConfigError> {
        self.log.clone().try_into()
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

    #[cfg(feature = "server")]
    pub async fn into_server_context(
        self,
        app_name: &'static str,
    ) -> Result<ServerContext, ParsedConfigError>
    {
        use caretta_sync_core::context::ServiceContext;

        let config = self.as_ref();
        let ipc_config = config.to_ipc_config()?;
        let p2p_config = config.to_p2p_config()?;
        let storage_config = config.to_storage_config()?;
        let iroh_router = p2p_config.to_iroh_router(app_name).await.unwrap();
        let local_database = storage_config.to_local_database();
        let cache_database = storage_config.to_cache_database();
        let service_context = ServiceContext {
            app_name,
            storage_config,
            iroh_router,
            local_database,
            cache_database,
        };
        Ok(ServerContext {
            app_name,
            ipc_config,
            service_context,
        })
    }
    #[cfg(feature = "client")]
    pub fn into_client_context(
        self,
        app_name: &'static str,
    ) -> Result<ClientContext, ParsedConfigError> {
        let config = self.as_ref();
        let ipc_config = config.to_ipc_config()?;
        Ok(ClientContext {
            app_name,
            ipc_config,
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
            p2p: ParsedP2pConfig::empty(),
            storage: ParsedStorageConfig::empty(),
            ipc: ParsedIpcConfig::empty(),
            log: ParsedLogConfig::empty(),
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.ipc.is_empty() && self.storage.is_empty() && self.log.is_empty()
    }
}

impl Mergeable for ParsedConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.ipc.merge(other.ipc);
        self.storage.merge(other.storage);
        self.log.merge(other.log);
    }
}

impl Display for ParsedConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}
impl Mergeable<ParsedP2pConfig> for ParsedConfig {
    fn merge(&mut self, other: ParsedP2pConfig) {
        self.p2p.merge(other);
    }
}
