use caretta_sync_demo_core::server::Server;
use clap::{Parser, Subcommand};
use caretta_sync::{cli::*, utils::runnable::Runnable};


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