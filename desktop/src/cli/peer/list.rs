use clap::Args;
use crate::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct PeerListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for PeerListCommandArgs {
    async fn run(self) {
        todo!()
    }
}