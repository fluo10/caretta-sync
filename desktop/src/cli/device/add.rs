use clap::Args;
use caretta_core::utils::runnable::Runnable;

use crate::cli::ConfigArgs;

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

impl Runnable for DeviceAddCommandArgs {
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}


