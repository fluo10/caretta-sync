use crate::cli::ConfigArgs;
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
}

impl Runnable for DeviceListCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
