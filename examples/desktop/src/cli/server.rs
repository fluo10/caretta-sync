use clap::Args;
use caretta::{error::Error, utils::runnable::Runnable};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl Runnable for ServerCommandArgs {
    async fn run(self) {
        let swarm_handler = P2P_CONFIG.get_and_unwrap().clone().launch_swarm();
        let server_handler = caretta_example_core::rpc::server::start_server();

        let (swarm_result, server_result) = tokio::try_join!(swarm_handler, server_handler).unwrap();
    }
}