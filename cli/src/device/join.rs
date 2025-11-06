use crate::{RunnableCommand, option::{ConfigOptionArgs, DeviceIdentifierArgs}};
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceJoinCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
}

impl RunnableCommand for DeviceJoinCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
