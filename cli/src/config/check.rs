use std::marker::PhantomData;

use crate::{RunnableCommand, option::ConfigOptionArgs};
use caretta_sync_core::{config::ParsedConfig, context::{ClientContext, ServerContext}};
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

impl<M> RunnableCommand for ConfigCheckCommandArgs<M>
where
    M: MigratorTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self.config.into_parsed_config(app_name).with_default(app_name).with_database(self.migrator).await.unwrap();

        let _ = ServerContext::from_parsed_config(config.clone(), self.migrator).await.unwrap();
        let _ = ClientContext::from_parsed_config(config).unwrap();
        println!("Ok");
    }
}
