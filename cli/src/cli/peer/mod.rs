mod info;
mod list;
mod ping;

pub use info::*;
pub use list::*;
pub use ping::*;

use caretta_sync_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct PeerCommandArgs {
    #[command(subcommand)]
    pub command: PeerSubcommand
}

impl Runnable for PeerCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
pub enum PeerSubcommand {
    Info(PeerInfoCommandArgs),
    List(PeerListCommandArgs),
    Ping(PeerPingCommandArgs),
}

impl Runnable for PeerSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::Info(x) => x.run(app_name),
            Self::List(x) => x.run(app_name),
            Self::Ping(x) => x.run(app_name),
        }
    }
}


