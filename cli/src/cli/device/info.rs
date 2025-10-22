use crate::cli::{ ConfigArgs, DeviceIdentifierArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceInfoCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
}

impl Runnable for DeviceInfoCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
