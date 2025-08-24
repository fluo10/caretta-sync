use caretta_sync_example_core::server::Server;
use clap::{Parser, Subcommand};
use caretta::{cli::*, utils::runnable::Runnable};


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
    Serve(ServeCommandArgs<Server>),
}