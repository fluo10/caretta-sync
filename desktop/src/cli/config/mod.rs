mod check;
mod list;

pub use check::*;
pub use list::*;

use caretta_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    pub command: ConfigSubcommand
}

impl Runnable for ConfigCommandArgs {
    async fn run(self) {
        self.command.run().await
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    Check(ConfigCheckCommandArgs),
    List(ConfigListCommandArgs),
}

impl Runnable for ConfigSubcommand {
    async fn run(self) {
        match self {
            Self::Check(x) => x.run().await,
            Self::List(x) => x.run().await,
        }
    }
}


