mod server;
use clap::{Parser, Subcommand};
use caretta::{cli::*, utils::runnable::Runnable};
pub use server::*;


#[derive(Debug, Parser, Runnable)]
pub struct Cli {
    #[command(subcommand)]
    #[runnable]
    command: CliCommand
}

#[derive(Debug, Subcommand, Runnable)]
pub enum CliCommand {
    Config(ConfigCommandArgs),
    Device(DeviceCommandArgs),
    Logs(LogsCommandArgs),
    Peer(PeerCommandArgs),
    Server(ServerCommandArgs),
}