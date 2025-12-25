use std::marker::PhantomData;

use caretta_framework_core::util::RunnableCommand;
use clap::Args;

use crate::{
    args::option::ConfigOptionArgs,
    config::parsed::ParsedConfig,
    types::Verbosity,
};

#[derive(Debug, Args)]
pub struct ConfigClientCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[arg(default_value_t)]
    verbosity: Verbosity,
}

impl RunnableCommand for ConfigClientCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut config = self.config.into_parsed_config(app_name);
        let mut config_to_print: Option<ParsedConfig> = None;
        if self.verbosity == Verbosity::Default {
            let _ = config_to_print.insert(config.clone().except_server_only_config());
        }

        config = config.with_default(app_name);
        if self.verbosity == Verbosity::Verbose {
            let _ = config_to_print.insert(config.clone().except_server_only_config());
        }
        
        let _ = config.into_client_context(app_name); 
        if let Some(x) = config_to_print {
            println!("{}", x)
        }
    }
}
