use crate::option::{ConfigOptionArgs, DeviceIdentifierArgs};
use crate::RunnableCommand;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs {
    #[command(flatten)]
    device: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl RunnableCommand for DeviceRemoveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
