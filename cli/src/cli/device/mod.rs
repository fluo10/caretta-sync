mod authorize;
mod info;
mod list;
mod ping;
mod remove;
mod scan;

use caretta_sync_core::utils::runnable::Runnable;
pub use info::DeviceInfoCommandArgs;
pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
pub use remove::DeviceRemoveCommandArgs;
pub use scan::DeviceScanCommandArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    pub command: DeviceSubcommand,
}

impl Runnable for DeviceCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
pub enum DeviceSubcommand {
    Info(DeviceInfoCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
    Scan(DeviceScanCommandArgs),
}

impl Runnable for DeviceSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::Info(x) => x.run(app_name),
            Self::List(x) => x.run(app_name),
            Self::Ping(x) => x.run(app_name),
            Self::Remove(x) => x.run(app_name),
            Self::Scan(x) => x.run(app_name),
        }
    }
}
