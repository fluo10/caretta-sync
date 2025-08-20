use clap::Args;
use caretta_core::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DeviceListCommandArgs {
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}