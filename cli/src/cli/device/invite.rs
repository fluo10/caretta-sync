use crate::cli::{ConfigOptionArgs, DeviceIdentifierArgs, DurationOptionArgs};
use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceInviteCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    duration: DurationOptionArgs,
}

impl Runnable for DeviceInviteCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
