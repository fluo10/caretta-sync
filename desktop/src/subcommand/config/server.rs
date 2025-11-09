use std::marker::PhantomData;


use caretta_sync_core::util::RunnableCommand;
use clap::Args;
use sea_orm_migration::MigratorTrait;

use crate::args::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigServerCommandArgs
{
    #[command(flatten)]
    config: ConfigArgs,
    /// Include default config.
    #[arg(short, long)]
    all: bool,
}

impl RunnableCommand for ConfigServerCommandArgs
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}
