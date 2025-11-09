
use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::TokenIdentifierArgs;

/// List up invitaion tokens
#[derive(Args, Debug)]
pub struct TokenListCommandArgs {
}

impl RunnableCommand for TokenListCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
