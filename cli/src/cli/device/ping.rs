use clap::Args;
use caretta_sync_core::utils::runnable::Runnable;
use crate::cli::{ConfigArgs, PeerArgs};

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs{
    #[command(flatten)]
    peer: PeerArgs,
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for DevicePingCommandArgs {
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}