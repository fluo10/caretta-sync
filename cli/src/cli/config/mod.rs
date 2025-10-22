mod check;
mod list;

pub use check::*;
pub use list::*;

use caretta_sync_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

impl Runnable for ConfigCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    Check(ConfigCheckCommandArgs),
    List(ConfigListCommandArgs),
}

impl Runnable for ConfigSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::Check(x) => x.run(app_name),
            Self::List(x) => x.run(app_name),
        }
    }
}
