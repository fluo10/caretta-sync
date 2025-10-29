use caretta_sync_core::utils::runnable::Runnable;
use clap::Args;

use crate::cli::args::VerificationIdentifierArgs;

/// Approve an authorization request
#[derive(Args, Debug)]
struct RejectCommandArgs {
    #[command(flatten)]
    verification: VerificationIdentifierArgs,
}

impl Runnable for RejectCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
