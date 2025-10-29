use crate::cli::{ConfigArgs, DeviceIdentifierArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceJoinCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
}

impl Runnable for DeviceJoinCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
