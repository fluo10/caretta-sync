use clap::Args;
use crate::utils::runnable::Runnable;
use crate::cli::{ConfigArgs, RunnableCommand};

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DeviceListCommandArgs {
    async fn run(self) {
        todo!()
    }
}