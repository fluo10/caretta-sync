use clap::Args;
use caretta_sync_core::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct DeviceScanCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DeviceScanCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}