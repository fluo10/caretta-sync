use crate::{
    config::{IrohConfig, RpcConfig},
    error::Error,
};

pub trait ServerTrait {
    async fn serve_p2p<T>(config: &T) -> Result<(), Error>
    where
        T: AsRef<IrohConfig>;
    async fn serve_rpc<T>(config: &T) -> Result<(), Error>
    where
        T: AsRef<RpcConfig>;
    async fn serve_all<T>(config: &T) -> Result<(), Error>
    where
        T: AsRef<IrohConfig> + AsRef<RpcConfig>,
    {
        tokio::try_join!(Self::serve_p2p(config), Self::serve_rpc(config))?;
        Ok(())
    }
}
