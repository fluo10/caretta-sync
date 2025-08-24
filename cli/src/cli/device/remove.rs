use clap::Args;
use caretta_sync_core::utils::runnable::Runnable;
use crate::cli::{ConfigArgs, DeviceArgs};

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs{
    #[command(flatten)]
    device: DeviceArgs,
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DeviceRemoveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}