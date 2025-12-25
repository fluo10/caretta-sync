use crate::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigOptionArgs, DeviceIdentifierOptionArgs, DurationArgs};

#[derive(Debug, Args)]
pub struct DeviceInviteCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    duration: DurationArgs,
}

impl RunnableCommand for DeviceInviteCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
