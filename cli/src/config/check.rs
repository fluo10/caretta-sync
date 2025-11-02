use std::marker::PhantomData;

use crate::option::ConfigOptionArgs;
use caretta_sync_core::{config::ParsedConfig, utils::runnable::Runnable};
use clap::Args;
use sea_orm_migration::MigratorTrait;

#[derive(Debug, Args)]
pub struct ConfigCheckCommandArgs<M>
where 
    M: MigratorTrait
{
    #[arg(skip)]
    migrator: PhantomData<M>,
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl<M> Runnable for ConfigCheckCommandArgs<M>
where
    M: MigratorTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let _ = self.config.clone().into_server_context(app_name, PhantomData::<M>).await;
        let _ = self.config.into_client_context(app_name);
        println!("Ok");
    }
}
