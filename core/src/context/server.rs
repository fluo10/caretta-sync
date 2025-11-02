use std::marker::PhantomData;

use iroh::Endpoint;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::{config::{LogConfig, P2pConfig, ParsedConfig, RpcConfig, StorageConfig}, error::Error};

#[derive(Clone, Debug)]
pub struct ServerContext {
    pub rpc_config: RpcConfig,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_endpoint: Option<Endpoint>,
    pub log_config: LogConfig,
}
impl ServerContext {
    pub async fn from_parsed_config<T,M>(config: T, _: PhantomData<M>) -> Result<Self, Error> 
    where
        T: AsRef<ParsedConfig>,
        M: MigratorTrait,
    {
        let config = config.as_ref();
        let rpc_config = config.to_rpc_config()?;
        let p2p_config = config.to_p2p_config(PhantomData::<M>).await?;
        let storage_config = config.to_storage_config()?;
        let database_connection = storage_config.to_database_connection::<M>().await?;
        let iroh_endpoint = p2p_config.to_endpoint().await?;
        let log_config = config.to_log_config()?;
        Ok(Self { rpc_config, storage_config, database_connection, iroh_endpoint, log_config })
    }

    pub fn init_tracing_subscriber(&self) {
        self.log_config.init_tracing_subscriber();
    }

}

impl AsRef<DatabaseConnection> for ServerContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}