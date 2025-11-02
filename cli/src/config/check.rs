use crate::option::ConfigOptionArgs;
use caretta_sync_core::{config::ParsedConfig, utils::runnable::Runnable};
use clap::Args;

#[derive(Debug, Args)]
pub struct ConfigCheckCommandArgs{
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl Runnable for ConfigCheckCommandArgs
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let _ = self.config.clone().into_server_context(app_name).await;
        let _ = self.config.into_client_context(app_name);
        println!("Ok");
    }
}
