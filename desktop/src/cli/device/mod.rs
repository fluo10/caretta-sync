mod add;
mod list;
mod ping;
mod remove;
mod scan;

pub use add::DeviceAddCommandArgs;
use crate::utils::runnable::Runnable;
use libp2p::{Multiaddr, PeerId};
pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
pub use remove::DeviceRemoveCommandArgs;
pub use scan::DeviceScanCommandArgs;

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Args, Runnable)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    #[runnable]
    pub command: DeviceSubcommand
}

#[derive(Debug, Subcommand, Runnable)]
pub enum DeviceSubcommand {
    Add(DeviceAddCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
    Scan(DeviceScanCommandArgs),
}

