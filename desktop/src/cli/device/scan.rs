use clap::Args;
use crate::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct DeviceScanCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DeviceScanCommandArgs {
    async fn run(self) {
        todo!()
    }
}