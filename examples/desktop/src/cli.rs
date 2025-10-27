use caretta_sync_example_core::{models::migration::Migrator, server::Server};
#[cfg(feature = "gui")]
mod gui;

use caretta_sync::{cli::*, config::Config, global::CONFIG, utils::Runnable};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
    #[command(flatten)]
    config: ConfigArgs,
}

impl Runnable for Cli {
    fn run(self, app_name: &'static str) {
        if let Some(x) = self.command {
            x.run(app_name)
        } else {
            #[cfg(feature = "gui")]
            gui::main();
            #[cfg(not(feature = "gui"))]
            todo!()
        }
    }
}

#[derive(Debug, Subcommand, Runnable)]
pub enum CliCommand {
    Config(ConfigCommandArgs),
    Device(DeviceCommandArgs),
    Peer(DeviceCommandArgs),
    Serve(ServeCommandArgs<Migrator, Server>),
}
