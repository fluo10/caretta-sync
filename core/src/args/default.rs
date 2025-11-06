use clap::Args;

use crate::{args::ConfigArgs, utils::RunnableCommand};

/// Default parser for GUI Client/server.
///
/// # Example
/// ```
/// # use caretta_sync_core::args::DefaultArgs;
/// use clap::Parser;
///
/// #[derive(Debug, Parser)]
/// struct Foo{
///     #[command(flatten)]
///     args: DefaultArgs,
/// }
/// ```
#[derive(Debug, Args)]
pub struct DefaultArgs {
    #[command(flatten)]
    config: ConfigArgs,
    /// Only checks config file and exits code 0 if no error was found.
    #[arg(short, long)]
    check: bool,
}

impl RunnableCommand for DefaultArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}