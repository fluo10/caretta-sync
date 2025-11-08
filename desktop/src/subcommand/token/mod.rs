mod list;
mod reject;
mod request;

#[derive(Debug, Args)]
pub struct TokenCommandArgs {
    #[command(subcommand)]
    command: TokenSubcommand,
}

impl RunnableCommand for TokenCommandArgs {
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
enum TokenSubcommand {
    Approve(TokenApprove),
    Invite(DeviceInviteCommandArgs),
    Join(DeviceJoinCommandArgs),
    List(DeviceListCommandArgs),
    Ping(DevicePingCommandArgs),
    Remove(DeviceRemoveCommandArgs),
}

impl RunnableCommand for DeviceSubcommand {
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