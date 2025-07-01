use clap::Args;

use crate::cli::{ConfigArgs, PeerArgs, RunnableCommand};

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs{
    #[command(flatten)]
    peer: PeerArgs,
    #[command(flatten)]
    config: ConfigArgs
}

impl RunnableCommand for DevicePingCommandArgs {
    async fn run(self) {
        todo!()
    }
}