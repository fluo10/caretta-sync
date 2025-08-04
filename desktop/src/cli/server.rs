use clap::Args;
use caretta_core::utils::runnable::Runnable;
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use crate::{error::Error, global::P2P_CONFIG};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl Runnable for ServerCommandArgs {
    async fn run(self) {
        P2P_CONFIG.get_and_unwrap().clone().launch_swarm();
    }
}