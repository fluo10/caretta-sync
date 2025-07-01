use clap::Args;

use crate::cli::{ConfigArgs, RunnableCommand};

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl RunnableCommand for DeviceListCommandArgs {
    async fn run(self) {
        todo!()
    }
}