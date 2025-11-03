use std::marker::PhantomData;

use iroh::{Endpoint, protocol::Router};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::{config::{LogConfig, P2pConfig, ParsedConfig, RpcConfig, StorageConfig}, error::Error};

#[derive(Clone, Debug)]
pub struct ServerContext {
    pub app_name: &'static str,
    pub rpc_config: RpcConfig,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_router: Option<Router>,
}
impl ServerContext {
    pub async fn new<T,M>(app_name: &'static str, config: T, migrator: PhantomData<M>) -> Result<Self, Error> 
    where
        T: AsRef<ParsedConfig>,
        M: MigratorTrait,
    {
        let config = config.as_ref();
        let rpc_config = config.to_rpc_config()?;
        let p2p_config = config.to_p2p_config()?;
        let storage_config = config.to_storage_config()?;
        let database_connection = storage_config.to_database_connection(migrator).await?;
        let iroh_router = p2p_config.to_iroh_router(app_name).await?;
        Ok(Self {app_name, rpc_config, storage_config, database_connection, iroh_router})
    }
}

impl AsRef<DatabaseConnection> for ServerContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}