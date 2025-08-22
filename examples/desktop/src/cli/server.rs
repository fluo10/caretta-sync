use caretta_example_core::server::Server;
use clap::Args;
use caretta::{config::Config, data::migration::DataMigrator, global::{CONFIG, DATABASE_CONNECTIONS}, server::ServerTrait, utils::runnable::Runnable};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl Runnable for ServerCommandArgs {
    async fn run(self, app_name: &'static str) {
        let config = CONFIG.get_or_init::<Config>(self.config.into_config(app_name).await).await;
        let _ = DATABASE_CONNECTIONS.get_or_init_unchecked(&config, DataMigrator).await;
        Server::serve_all(config).await.unwrap();
    }
}