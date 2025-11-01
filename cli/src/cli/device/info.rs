use crate::cli::{ConfigOptionArgs, DeviceIdentifierArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceInfoCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
}

impl Runnable for DeviceInfoCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
