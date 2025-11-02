mod info;
mod invite;
mod join;
mod list;
mod ping;
mod remove;

use caretta_sync_core::utils::runnable::Runnable;
pub use info::DeviceInfoCommandArgs;
pub use invite::DeviceInviteCommandArgs;
pub use join::DeviceJoinCommandArgs;
pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
pub use remove::DeviceRemoveCommandArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    command: DeviceSubcommand,
}

impl Runnable for DeviceCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
enum DeviceSubcommand {
    Info(DeviceInfoCommandArgs),
    Invite(DeviceInviteCommandArgs),
    Join(DeviceJoinCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
}

impl Runnable for DeviceSubcommand {
    fn run(self, app_name: &'static str) {
        match self {
            Self::Info(x) => x.run(app_name),
            Self::Invite(x) => x.run(app_name),
            Self::Join(x) => x.run(app_name),
            Self::List(x) => x.run(app_name),
            Self::Ping(x) => x.run(app_name),
            Self::Remove(x) => x.run(app_name),
        }
    }
}
