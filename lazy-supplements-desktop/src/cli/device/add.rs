use clap::Args;

use crate::cli::{ConfigArgs, RunnableCommand};

use crate::cli::PeerArgs;

#[derive(Debug, Args)]
pub struct DeviceAddCommandArgs {
    #[command(flatten)]
    peer: PeerArgs,
    #[arg(short, long)]
    passcode: Option<String>,
    #[command(flatten)]
    config: ConfigArgs
}

impl RunnableCommand for DeviceAddCommandArgs {
    async fn run(self) {
        todo!()
    }
}


