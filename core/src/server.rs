use crate::{config::{Config, P2pConfig, RpcConfig}, error::Error};

pub trait ServerTrait {
    async fn serve_p2p(config: &P2pConfig) -> Result<(), Error>;
    async fn serve_rpc(config: &RpcConfig) -> Result<(), Error>;
    async fn serve_all(config: &Config) -> Result<(), Error> {
        tokio::try_join!(
            Self::serve_p2p(&config.p2p),
            Self::serve_rpc(&config.rpc)
        )?;
        Ok(())
    }
}