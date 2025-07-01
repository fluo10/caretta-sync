mod add;
mod list;
mod ping;
mod remove;
mod scan;

pub use add::DeviceAddCommandArgs;
use libp2p::{Multiaddr, PeerId};
pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
pub use remove::DeviceRemoveCommandArgs;
pub use scan::DeviceScanCommandArgs;

use std::{net::IpAddr, ops::Mul, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};

use crate::{cli::ServerArgs, error::Error};

use super::ConfigArgs;


#[derive(Debug, Args)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    pub command: DeviceSubcommand
}

#[derive(Debug, Subcommand)]
pub enum DeviceSubcommand {
    Add(DeviceAddCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
    Scan(DeviceScanCommandArgs),
}

