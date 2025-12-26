// mod info;
// mod invite;
// mod join;
// mod list;
mod ping;
// mod remove;

use crate::{types::AppInfo, util::RunnableCommand};
// pub use info::DeviceInfoCommandArgs;
// pub use invite::DeviceInviteCommandArgs;
// pub use join::DeviceJoinCommandArgs;
// pub use list::DeviceListCommandArgs;
pub use ping::DevicePingCommandArgs;
// pub use remove::DeviceRemoveCommandArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DeviceCommandArgs {
    #[command(subcommand)]
    command: DeviceSubcommand,
}

impl RunnableCommand for DeviceCommandArgs {
    fn run(self, app_info: AppInfo) {
        self.command.run(app_info)
    }
}

#[derive(Debug, Subcommand)]
enum DeviceSubcommand {
    // Info(DeviceInfoCommandArgs),
    // Invite(DeviceInviteCommandArgs),
    // Join(DeviceJoinCommandArgs),
    // List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    // Remove(DeviceRemoveCommandArgs),
}

impl RunnableCommand for DeviceSubcommand {
    fn run(self, app_info: AppInfo) {
        match self {
            // Self::Info(x) => x.run(app_name),
            // Self::Invite(x) => x.run(app_name),
            // Self::Join(x) => x.run(app_name),
            // Self::List(x) => x.run(app_name),
            Self::Ping(x) => x.run(app_info),
            // Self::Remove(x) => x.run(app_name),
        }
    }
}
