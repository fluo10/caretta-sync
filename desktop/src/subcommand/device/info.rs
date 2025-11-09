use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs};

#[derive(Debug, Args)]
pub struct DeviceInfoCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    peer: DeviceIdentifierArgs,
}

impl RunnableCommand for DeviceInfoCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
