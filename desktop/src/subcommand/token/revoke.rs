use caretta_sync_core::utils::RunnableCommand;
use clap::Args;

use crate::args::TokenIdentifierArgs;


/// Approve an authorization request
#[derive(Args, Debug)]
pub struct TokenRevokeCommandArgs {
    #[command(flatten)]
    target: TokenIdentifierArgs,
}

impl RunnableCommand for TokenRevokeCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
