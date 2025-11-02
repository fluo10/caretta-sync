use caretta_sync_core::utils::runnable::RunnableCommand;
use clap::Args;
use mtid::Dtid;

use crate::cli::args::VerificationIdentifierArgs;

/// Approve an authorization request
#[derive(Args, Debug)]
struct ApproveCommandArgs {
    #[command(flatten)]
    authorization_request: VerificationIdentifierArgs,
    passcode: Dtid,
}

impl RunnableCommand for ApproveCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
