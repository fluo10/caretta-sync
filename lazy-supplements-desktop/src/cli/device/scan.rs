use clap::Args;

use crate::cli::{ConfigArgs, RunnableCommand};

#[derive(Debug, Args)]
pub struct DeviceScanCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl RunnableCommand for DeviceScanCommandArgs {
    async fn run(self) {
        todo!()
    }
}