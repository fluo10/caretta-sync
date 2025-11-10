mod common;
use common::APP_NAME;

use caretta_sync::{
    util::RunnableCommand, 
    subcommand::{ConfigCommandArgs, DeviceCommandArgs, TokenCommandArgs},
};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

impl RunnableCommand for Cli {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand, RunnableCommand)]
pub enum CliCommand {
    Config(ConfigCommandArgs),
    Device(DeviceCommandArgs),
    Token(TokenCommandArgs),
}

fn main() {
    let args = Cli::parse();
    args.run(APP_NAME)
}