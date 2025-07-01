use clap::Args;

use crate::cli::{ConfigArgs, DeviceArgs, RunnableCommand};

#[derive(Debug, Args)]
pub struct DeviceRemoveCommandArgs{
    #[command(flatten)]
    device: DeviceArgs,
    #[command(flatten)]
    config: ConfigArgs
}

impl RunnableCommand for DeviceRemoveCommandArgs {
    async fn run(self) {
        todo!()
    }
}