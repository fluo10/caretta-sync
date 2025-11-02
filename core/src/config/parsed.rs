use base64::engine::Config;
use sea_orm_migration::MigratorTrait;
use serde::{ser::Error, Deserialize, Serialize};

use crate::{config::{ConfigError, P2pConfig, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, RpcConfig, StorageConfig}, context::{ClientContext, ServerContext}, models::P2pConfigModel, utils::{emptiable::Emptiable, mergeable::Mergeable}};
use std::{fmt::{Display, write}, fs::File, io::Read, marker::PhantomData, path::{Path, PathBuf}};

#[cfg_attr(feature="cli", derive(clap::Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParsedConfig {
    #[cfg_attr(feature="cli", command(flatten))]
    pub storage: Option<PartialStorageConfig>,
    #[cfg_attr(feature="cli", command(flatten))]
    pub rpc: Option<PartialRpcConfig>,
    #[cfg_attr(feature="cli", command(flatten))]
    pub p2p: Option<PartialP2pConfig>,
}



/// A partial config parsed from config file, cli args, etc.
impl ParsedConfig {

    pub fn default(app_name: &'static str) -> Self {
        Self {
            storage: Some(PartialStorageConfig::default(app_name)),
            rpc: Some(PartialRpcConfig::default(app_name)),
            p2p: None,
        }
    }

    /// Build [`StorageConfig`] from own [`PartialStorageConfig`]
    pub fn to_storage_config(&self) -> Result<StorageConfig, ConfigError> {
        self.storage.as_ref().ok_or(ConfigError::MissingConfig("[storage]"))?.clone().try_into()
    }

    /// Build [`P2pConfig`] from own [`PartialP2pConfig`]
    pub async fn to_p2p_config<T>(&self, _: PhantomData<T>) -> Result<P2pConfig, ConfigError>
    where T: MigratorTrait
    {
        let p2p_config = self.p2p.clone().unwrap_or(PartialP2pConfig::empty());
        let storage_config = self.to_storage_config()?;
        let connection =  storage_config.to_database_connection::<T>().await?;
        let mut config = PartialP2pConfig::from(P2pConfig::from(P2pConfigModel::get_or_try_init(&connection).await?));
        config.merge(p2p_config);
        Ok(P2pConfig::try_from(config)?)
    }

    /// Build [`RpcConfig`] from own [`PartialRpcConfig`]
    pub fn to_rpc_config(&self) -> Result<RpcConfig, ConfigError> {
        self.rpc.as_ref().ok_or(ConfigError::MissingConfig("rpc.*"))?.clone().try_into()
    }

    /// Read or create target config file
    pub fn read_or_create_from_path<T>(path: T) -> Result<Self, ConfigError>
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
    pub fn default_config_path(app_name: &'static str) -> Result<PathBuf, ConfigError> {
        const DEFAULT_FILE_NAME: &str = "config.toml";
        let mut path = dirs::config_local_dir().ok_or(ConfigError::ConfigDir)?;
        path.push(app_name);
        path.push(DEFAULT_FILE_NAME);
        Ok(path)
    }

    /// Read or create target config file at the default config path
    pub fn read_or_create(app_name: &'static str) -> Result<Self, ConfigError> {
        let config_dir = Self::default_config_path(app_name)?;
        Self::read_or_create_from_path(config_dir)
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
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.rpc.is_empty() && self.storage.is_empty()
    }
}

impl Mergeable for ParsedConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.rpc.merge(other.rpc);
        self.storage.merge(other.storage);
    }
}

impl Display for ParsedConfig{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string(self).map_err(|_| std::fmt::Error)?)
    }

}
