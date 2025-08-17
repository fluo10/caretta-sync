mod server;
use clap::{Parser, Subcommand};
use caretta::cli::*;
pub use server::*;


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