#[cfg(feature = "gui")]
use caretta_sync_example_core::gui::Gui;
use caretta_sync_example_core::{models::migration::Migrator, server::Server};
#[cfg(feature = "gui")]
mod gui;

use caretta_sync::cli::{option::ConfigOptionArgs,RunnableCommand, *};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
    #[command(flatten)]
    pub config: ConfigOptionArgs,
}

impl RunnableCommand for Cli {
    fn run(self, app_name: &'static str) {
        if let Some(x) = self.command {
            x.run(app_name)
        } else {
            #[cfg(feature = "gui")]
            Gui{}.run(app_name);
            #[cfg(not(feature = "gui"))]
            todo!()
        }
    }
}

#[derive(Debug, Subcommand, RunnableCommand)]
pub enum CliCommand {
    Config(ConfigCommandArgs<Migrator>),
    Device(DeviceCommandArgs),
    Serve(ServeCommandArgs<Migrator, Server>),
}
