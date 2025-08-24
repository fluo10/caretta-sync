mod check;
mod list;

pub use check::*;
pub use list::*;

use caretta_sync_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    pub command: ConfigSubcommand
}

impl Runnable for ConfigCommandArgs {
    async fn run(self, app_name: &'static str) {
        self.command.run(app_name).await
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    Check(ConfigCheckCommandArgs),
    List(ConfigListCommandArgs),
}

impl Runnable for ConfigSubcommand {
    async fn run(self, app_name: &'static str) {
        match self {
            Self::Check(x) => x.run(app_name).await,
            Self::List(x) => x.run(app_name).await,
        }
    }
}


