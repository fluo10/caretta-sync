use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs, DurationArgs};

#[derive(Debug, Args)]
pub struct DeviceInviteCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    duration: DurationArgs,
}

impl RunnableCommand for DeviceInviteCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
