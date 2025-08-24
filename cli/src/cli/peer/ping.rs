use clap::Args;
use caretta_sync_core::utils::runnable::Runnable;
use crate::cli::{ConfigArgs, PeerArgs};

#[derive(Debug, Args)]
pub struct PeerPingCommandArgs{
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    peer: PeerArgs,
}

impl Runnable for PeerPingCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}