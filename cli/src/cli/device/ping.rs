use crate::cli::{ConfigArgs, args::DeviceIdentifierArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs {
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigArgs,
}

impl Runnable for DevicePingCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
