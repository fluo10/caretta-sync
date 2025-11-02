use std::marker::PhantomData;

use crate::option::ConfigOptionArgs;
use caretta_sync_core::{config::ParsedConfig, utils::runnable::Runnable};
use clap::Args;
use sea_orm_migration::MigratorTrait;

#[derive(Debug, Args)]
pub struct ConfigListCommandArgs<M>
where 
    M: MigratorTrait
{
    #[arg(skip)]
    migrator: PhantomData<M>,
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[arg(short, long)]
    all: bool,
}

impl<M> Runnable for ConfigListCommandArgs<M>
where 
    M: MigratorTrait
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut config = self.config.into_parsed_config(app_name);
         if self.all {            
            config = ParsedConfig {
                storage: Some(config.to_storage_config().unwrap().into()),
                p2p: Some(config.to_p2p_config(PhantomData::<M>).await.unwrap().into()),
                rpc: Some(config.to_rpc_config().unwrap().into())
            }
        };
        println!("{}", config)
    }
}
