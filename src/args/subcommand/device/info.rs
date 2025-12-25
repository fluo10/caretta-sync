use crate::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigOptionArgs, DeviceIdentifierOptionArgs};

#[derive(Debug, Args)]
pub struct DeviceInfoCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    peer: DeviceIdentifierOptionArgs,
}

impl RunnableCommand for DeviceInfoCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
