use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::TokenIdentifierArgs;

/// Show infomation of an invitaion token
#[derive(Args, Debug)]
pub struct TokenShowCommandArgs {
    #[command(flatten)]
    target: TokenIdentifierArgs,
}

impl RunnableCommand for TokenShowCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
