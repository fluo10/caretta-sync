use caretta_sync_core::util::RunnableCommand;
use clap::{Args, Subcommand};

use crate::subcommand::token::{
    list::TokenListCommandArgs, revoke::TokenRevokeCommandArgs, show::TokenShowCommandArgs,
};

mod list;
mod revoke;
mod show;

#[derive(Debug, Args)]
pub struct TokenCommandArgs {
    #[command(subcommand)]
    command: TokenSubcommand,
}

impl RunnableCommand for TokenCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
enum TokenSubcommand {
    List(TokenListCommandArgs),
    Revoke(TokenRevokeCommandArgs),
    Show(TokenShowCommandArgs),
}

impl RunnableCommand for TokenSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::List(x) => x.run(app_name),
            Self::Revoke(x) => x.run(app_name),
            Self::Show(x) => x.run(app_name),
        }
    }
}
