use clap::{Parser, Subcommand};
use caretta_desktop::cli::*;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    //Config(ConfigCommandArgs),
    //Device(DeviceCommandArgs),
    //Log(LogCommandArgs),
    Server(ServerCommandArgs),
}