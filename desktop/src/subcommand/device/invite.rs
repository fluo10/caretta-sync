use crate::{RunnableCommand, option::{ConfigOptionArgs, DeviceIdentifierArgs, DurationOptionArgs}};
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceInviteCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    duration: DurationOptionArgs,
}

impl RunnableCommand for DeviceInviteCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
