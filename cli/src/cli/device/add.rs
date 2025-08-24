use clap::Args;
use caretta_sync_core::utils::runnable::Runnable;

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
    fn run(self, app_name: &'static str) {
        todo!()
    }
}


