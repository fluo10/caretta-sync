use std::marker::PhantomData;

use caretta_framework_core::util::RunnableCommand;
use clap::Args;

use crate::args::option::ConfigOptionArgs;

#[derive(Debug, Args)]
pub struct ConfigServerCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    /// Include default config.
    #[arg(short, long)]
    all: bool,
}

impl RunnableCommand for ConfigServerCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        todo!()
    }
}
