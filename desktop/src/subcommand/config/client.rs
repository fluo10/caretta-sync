use std::marker::PhantomData;

use caretta_sync_core::util::RunnableCommand;
use clap::Args;
use sea_orm_migration::MigratorTrait;

use crate::args::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigClientCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    /// Include default config.
    #[arg(short, long)]
    all: bool,
}

impl RunnableCommand for ConfigClientCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut config = self.config.into_parsed_config(app_name);
        if self.all {
            config = config.with_default(app_name);
        };
        config = config.except_server_only_config();
        println!("{}", config)
    }
}
