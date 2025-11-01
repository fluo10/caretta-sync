use sea_orm_migration::MigratorTrait;
use serde::{Deserialize, Serialize};

use crate::{config::{ConfigError, P2pConfig, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig, RpcConfig, StorageConfig}, context::{ClientContext, ServerContext}, models::P2pConfigModel, utils::{emptiable::Emptiable, mergeable::Mergeable}};
use std::{fs::File, io::Read, path::Path};

/// A partial config parsed from config file, cli args, etc.
pub trait ParsedConfig : for<'a> Deserialize<'a> + Serialize + Emptiable + Mergeable{

    /// A getter of [`PartialStorageConfig`]
    fn partial_storage_config(&self) -> Option<&PartialStorageConfig>;

    /// Build [`StorageConfig`] from own [`PartialStorageConfig`]
    fn to_storage_config(&self) -> Result<StorageConfig, ConfigError> {
        self.partial_storage_config().ok_or(ConfigError::MissingConfig("[storage]"))?.clone().try_into()
    }

    /// A getter of [`PartialP2pConfig`]
    fn partial_p2p_config(&self) -> Option<&PartialP2pConfig>;

    /// Build [`P2pConfig`] from own [`PartialP2pConfig`]
    async fn to_p2p_config<T>(&self) -> Result<P2pConfig, ConfigError>
    where T: MigratorTrait
    {
        let mut p2p_config = self.partial_p2p_config().cloned().unwrap_or(PartialP2pConfig::empty());
        let storage_config = self.to_storage_config()?;
        let connection =  storage_config.to_database_connection::<T>().await?;
        let mut config = PartialP2pConfig::from(P2pConfig::from(P2pConfigModel::get_or_try_init(&connection).await?));
        config.merge(p2p_config);
        Ok(P2pConfig::try_from(config)?)
    }

    /// A getter of [`PartialRpcConfig`]
    fn partial_rpc_config(&self) -> Option<&PartialRpcConfig>;

    /// Build [`RpcConfig`] from own [`PartialRpcConfig`]
    fn to_rpc_config(&self) -> Result<RpcConfig, ConfigError> {
        self.partial_rpc_config().ok_or(ConfigError::MissingConfig("rpc.*"))?.clone().try_into()
    }

    /// Read or create target config file
    fn read_or_create<T>(path: T) -> Result<Self, ConfigError>
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

    /// Default config.
    fn default(app_name: &'static str) -> Self;
}