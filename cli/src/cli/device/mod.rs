mod add;
mod list;
mod ping;
mod remove;
mod scan;

pub use add::DeviceAddCommandArgs;
use caretta_sync_core::utils::runnable::Runnable;
use libp2p::{Multiaddr, PeerId};
pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
pub use remove::DeviceRemoveCommandArgs;
pub use scan::DeviceScanCommandArgs;

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Args)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    pub command: DeviceSubcommand
}

impl Runnable for DeviceCommandArgs {
    async fn run(self, app_name: &'static str) {
        self.command.run(app_name).await
    }
}

#[derive(Debug, Subcommand)]
pub enum DeviceSubcommand {
    Add(DeviceAddCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
    Scan(DeviceScanCommandArgs),
}

impl Runnable for DeviceSubcommand {
    async fn run(self, app_name: &'static str) {
        match self {
            Self::Add(x) => x.run(app_name).await,
            Self::List(x) => x.run(app_name).await,
            Self::Ping(x) => x.run(app_name).await,
            Self::Remove(x) => x.run(app_name).await,
            Self::Scan(x) => x.run(app_name).await,
        }
    }
}

