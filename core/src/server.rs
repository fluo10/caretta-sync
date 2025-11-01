use async_trait::async_trait;
use iroh::discovery::dns::DnsDiscovery;
use sea_orm_migration::MigratorTrait;

use crate::{
    config::{P2pConfig, ParsedConfig, RpcConfig, StorageConfig},
    error::Error,
};

#[async_trait]
pub trait ServerTrait: Send + Sync {
    async fn serve<C, M>(config: &C) -> Result<(), Error>
    where
        C: ParsedConfig,
        M: MigratorTrait;
}
