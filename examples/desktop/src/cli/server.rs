use clap::Args;
use caretta::{config::Config, error::Error, global::CONFIG, utils::runnable::Runnable};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl Runnable for ServerCommandArgs {
    async fn run(self, app_name: &'static str) {
        let config = CONFIG.get_or_init::<Config>(self.config.into_config_unchecked(app_name).await).await;

    }
}