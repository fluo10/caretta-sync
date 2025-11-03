use std::marker::PhantomData;

use crate::{RunnableCommand, option::ConfigOptionArgs};
use caretta_sync_core::{config::ParsedConfig};
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

impl<M> RunnableCommand for ConfigListCommandArgs<M>
where 
    M: MigratorTrait
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut config = self.config.into_parsed_config(app_name);
        if self.all {
            config = config.with_default(app_name).with_database(self.migrator).await.unwrap();
        };
        println!("{}", config)
    }
}
