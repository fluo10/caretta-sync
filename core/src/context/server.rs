use iroh::Endpoint;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tonic::transport::Server;
use tracing_subscriber::registry::Data;

use crate::{config::{P2pConfig, ParsedConfig, RpcConfig, StorageConfig}, error::Error};

pub struct ServerContext {
    pub rpc_config: RpcConfig,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_endpoint: Option<Endpoint>,
}
impl ServerContext {
    pub async fn from_parsed_config<T,M>(config: T) -> Result<Self, Error> 
    where
        T: ParsedConfig,
        M: MigratorTrait,
    {
        let rpc_config = config.to_rpc_config()?;
        let p2p_config = config.to_p2p_config::<M>().await?;
        let storage_config = config.to_storage_config()?;
        let database_connection = storage_config.to_database_connection::<M>().await?;
        let iroh_endpoint = p2p_config.to_endpoint().await?;

        Ok(Self { rpc_config, storage_config, database_connection, iroh_endpoint })
    }

}

impl AsRef<DatabaseConnection> for ServerContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}