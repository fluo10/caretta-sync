use std::marker::PhantomData;

use clap::Args;
use caretta_core::{config::Config, data::migration::DataMigrator, global::{CONFIG, DATABASE_CONNECTIONS}, server::ServerTrait, utils::runnable::Runnable};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServeCommandArgs<T> 
where
    T: ServerTrait
{
    #[arg(skip)]
    server: PhantomData<T>,
    #[command(flatten)]
    config: ConfigArgs,
}
impl<T> Runnable for ServeCommandArgs<T>
where 
    T: ServerTrait
{
    async fn run(self, app_name: &'static str) {
        let config = CONFIG.get_or_init::<Config>(self.config.into_config(app_name).await).await;
        let _ = DATABASE_CONNECTIONS.get_or_init_unchecked(&config, DataMigrator).await;
        T::serve_all(config).await.unwrap();
    }
}