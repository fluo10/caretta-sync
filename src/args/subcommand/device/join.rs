use crate::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigOptionArgs, DeviceIdentifierOptionArgs};

#[derive(Debug, Args)]
pub struct DeviceJoinCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    peer: DeviceIdentifierOptionArgs,
}

impl RunnableCommand for DeviceJoinCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
