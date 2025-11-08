use caretta_sync_core::utils::runnable::RunnableCommand;
use clap::Args;

/// Show infomation of an invitaion token
#[derive(Args, Debug)]
struct TokenRevokeCommandArgs {
    #[command(flatten)]
    target: TokenIdentifierArgs,
}

impl RunnableCommand for RejectCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}
