use crate::option::ConfigOptionArgs;
use caretta_sync_core::{config::ParsedConfig, example::migrator::ExampleMigrator, utils::runnable::Runnable};
use clap::Args;

#[derive(Debug, Args)]
pub struct ConfigListCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[arg(short, long)]
    all: bool,
}

impl Runnable for ConfigListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut config = self.config.into_parsed_config(app_name);
         if self.all {            
            config = ParsedConfig {
                storage: Some(config.to_storage_config().unwrap().into()),
                p2p: Some(config.to_p2p_config::<ExampleMigrator>().await.unwrap().into()),
                rpc: Some(config.to_rpc_config().unwrap().into())
            }
        };
        println!("{}", config)
    }
}
