use clap::Args;
use crate::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigCheckCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for ConfigCheckCommandArgs {
    async fn run(self) {
        todo!()
    }
}