use clap::Args;
use caretta_sync_core::{config::PartialConfig, utils::runnable::Runnable};
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs,
    #[arg(short,long)]
    all: bool
}

impl Runnable for ConfigListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config: PartialConfig = if self.all {
            self.config.into_config(app_name).await.into()
        } else {
            self.config.to_partial_config_without_default(app_name).await
        };
        println!("{}", config.into_toml().unwrap())

    }
}