use clap::{Parser, Subcommand};
use lazy_supplements::{cli::ServerArgs, config::PartialServerConfig};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Add,
    Delete,
    List,
    Server(ServerArgs)
}
