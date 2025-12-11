use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs};

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs {
    #[command(flatten)]
    device: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigArgs,
}

impl RunnableCommand for DeviceRemoveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
