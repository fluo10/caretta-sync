use async_trait::async_trait;
use iroh::discovery::dns::DnsDiscovery;
use sea_orm_migration::MigratorTrait;

use crate::{
    config::{IrohConfig, RpcConfig, StorageConfig},
    error::Error,
    global::{IROH_ENDPOINT, LOCAL_DATABASE_CONNECTION},
};

#[async_trait]
pub trait ServerTrait: Send + Sync {
    async fn init_database<C, M>(config: &C) -> Result<(), Error>
    where
        C: AsRef<StorageConfig> + Send + Sync,
        M: MigratorTrait,
    {
        let _ = LOCAL_DATABASE_CONNECTION
            .get_or_try_init::<_, M>(&config.as_ref().get_local_database_path())
            .await?;
        Ok(())
    }

    async fn serve_p2p<T>(config: &T) -> Result<(), Error>
    where
        T: AsRef<IrohConfig> + Send + Sync,
    {
        let endpoint = iroh::Endpoint::builder()
            .discovery(DnsDiscovery::n0_dns())
            .bind()
            .await?;
        let _ = IROH_ENDPOINT.get_or_init(&endpoint);
        Ok(())
    }
    async fn serve_rpc<T>(config: &T) -> Result<(), Error>
    where
        T: AsRef<RpcConfig> + Send + Sync;
    async fn serve<C, M>(config: &C) -> Result<(), Error>
    where
        C: AsRef<IrohConfig> + AsRef<RpcConfig> + AsRef<StorageConfig> + Send + Sync,
        M: MigratorTrait,
    {
        Self::init_database::<_, M>(config).await?;
        tokio::try_join!(Self::serve_p2p(config), Self::serve_rpc(config))?;
        Ok(())
    }
}
