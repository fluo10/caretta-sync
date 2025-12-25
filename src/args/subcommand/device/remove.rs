use caretta_framework_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigOptionArgs, DeviceIdentifierOptionArgs};

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs {
    #[command(flatten)]
    device: DeviceIdentifierOptionArgs,
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl RunnableCommand for DeviceRemoveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
