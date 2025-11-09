use caretta_sync_core::util::RunnableCommand;
use clap::{Args, Subcommand};

mod client;
mod server;

use client::ConfigClientCommandArgs;
use server::ConfigServerCommandArgs;

#[derive(Debug, Args)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

impl RunnableCommand for ConfigCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
enum ConfigSubcommand {
    Client(ConfigClientCommandArgs),
    Server(ConfigServerCommandArgs),
}

impl RunnableCommand for ConfigSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::Client(x) => x.run(app_name),
            Self::Server(x) => x.run(app_name),
        }
    }
}