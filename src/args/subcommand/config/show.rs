use std::marker::PhantomData;

use crate::{config::parsed::ParsedConfig, types::AppInfo, util::RunnableCommand};
use clap::Args;

use crate::args::option::ConfigOptionArgs;

/// Print active config list.
#[derive(Debug, Args)]
pub struct ConfigShowCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    /// Include default config.
    #[arg(short, long)]
    all: bool,
}

impl RunnableCommand for ConfigShowCommandArgs {
    #[tokio::main]
    async fn run(self, app_info : AppInfo) {
        let app_name = app_info.name;
        let mut config = self.config.into_parsed_config(app_name);
        #[cfg(feature = "desktop-server")]
        {
            config = config.with_database().await;
        }
        if self.all {
            config = config.with_default(app_name);
        }
        println!("{}", config)
    }
}
