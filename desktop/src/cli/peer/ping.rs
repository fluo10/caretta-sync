use clap::Args;
use caretta_core::utils::runnable::Runnable;
use crate::cli::{ConfigArgs, PeerArgs};

#[derive(Debug, Args)]
pub struct PeerPingCommandArgs{
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    peer: PeerArgs,
}

impl Runnable for PeerPingCommandArgs {
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}