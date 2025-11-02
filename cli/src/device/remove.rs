use crate::option::{ConfigOptionArgs, DeviceIdentifierArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs {
    #[command(flatten)]
    device: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl Runnable for DeviceRemoveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
