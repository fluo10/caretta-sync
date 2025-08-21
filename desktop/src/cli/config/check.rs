use clap::Args;
use caretta_core::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigCheckCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for ConfigCheckCommandArgs {
    async fn run(self, app_name: &'static str) {
        let _ = self.config.into_config(app_name).await;
        println!("Ok");
    }
}