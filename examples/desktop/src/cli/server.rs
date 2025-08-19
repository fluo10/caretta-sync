use clap::Args;
use caretta::{error::Error, global::CONFIG, utils::runnable::Runnable};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl Runnable for ServerCommandArgs {
    async fn run(self) {
        let config = CONFIG.get_or_init(self.config.try_into()?).await;
    }
}