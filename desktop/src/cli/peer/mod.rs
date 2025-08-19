mod info;
mod list;
mod ping;

pub use info::*;
pub use list::*;
pub use ping::*;

use caretta_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct PeerCommandArgs {
    #[command(subcommand)]
    pub command: PeerSubcommand
}

impl Runnable for PeerCommandArgs {
    async fn run(self) {
        self.command.run().await
    }
}

#[derive(Debug, Subcommand)]
pub enum PeerSubcommand {
    Info(PeerInfoCommandArgs),
    List(PeerListCommandArgs),
    Ping(PeerPingCommandArgs),
}

impl Runnable for PeerSubcommand {
    async fn run(self) {
        match self {
            Self::Info(x) => x.run().await,
            Self::List(x) => x.run().await,
            Self::Ping(x) => x.run().await,
        }
    }
}


