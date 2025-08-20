use clap::Args;
use caretta_core::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct PeerListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for PeerListCommandArgs {
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}