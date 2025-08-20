use crate::{config::{Config, P2pConfig, RpcConfig}, error::Error};

pub trait ServerTrait {
    async fn serve_p2p<T>(config: &T) -> Result<(), Error>
    where T: AsRef<P2pConfig>;
    async fn serve_rpc<T>(config: &T) -> Result<(), Error>
    where T: AsRef<RpcConfig>;
    async fn serve_all<T>(config: &T) -> Result<(), Error>
    where 
        T: AsRef<P2pConfig> + AsRef<RpcConfig> {
        tokio::try_join!(
            Self::serve_p2p(config),
            Self::serve_rpc(config)
        )?;
        Ok(())
    }
}