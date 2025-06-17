use clap::Args;
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use crate::{error::Error, global::GLOBAL};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl ServerArgs {
    pub async fn start_server(self) -> Result<(), Error>{
        let _ = crate::global::GLOBAL.get_or_init_core_config(self.config.try_into_core_config().await?).await;
        GLOBAL.launch_swarm().await
    }
}