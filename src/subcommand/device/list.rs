use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs};
use caretta_sync_core::{
    context::ClientContext,
};
use caretta_id::CarettaId;

#[derive(Debug, Args)]
#[group(multiple = false)]
struct FilterOptionArgs {
    #[arg(long)]
    discovered: bool,
    #[arg(long)]
    all: bool,
}


#[derive(Debug, Args)]
pub struct DeviceListCommandArgs {
    #[command(flatten)]
    config: ConfigArgs,
    #[command(flatten)]
    filter: FilterOptionArgs,
    #[arg(short, long)]
    verbose: bool,
}

impl RunnableCommand for DeviceListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self.config.into_parsed_config(app_name);
        if self.verbose {
            config.init_tracing_subscriber();
        }
        let context = config.into_client_context(app_name).unwrap();
        todo!()
    }
}
