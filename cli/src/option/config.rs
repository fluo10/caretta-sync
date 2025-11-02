use std::path::PathBuf;

use caretta_sync_core::{
    config::ParsedConfig, context::{ClientContext, ServerContext}, example::migrator::ExampleMigrator, utils::{emptiable::Emptiable, mergeable::Mergeable}
};
use clap::Args;

use tokio::sync::OnceCell;

#[derive(Args, Clone, Debug)]
pub struct ConfigOptionArgs
{
    #[arg(short = 'c', long = "config")]
    pub file_path: Option<PathBuf>,
    #[command(flatten)]
    pub args: ParsedConfig,
}

impl ConfigOptionArgs
{
    pub fn into_parsed_config(self, app_name: &'static str) -> ParsedConfig {
        let mut config = match self.file_path {
            Some(x) => ParsedConfig::read_or_create_from_path(x).unwrap(),
            None => ParsedConfig::read_or_create(app_name).unwrap()
        };
        config.merge(self.args);
        config
    }

    pub fn into_client_context(self, app_name: &'static str) -> ClientContext {
        ClientContext::from_parsed_config(self.into_parsed_config(app_name)).unwrap()
    }
    pub async fn into_server_context(self, app_name: &'static str) -> ServerContext {
        ServerContext::from_parsed_config(self.into_parsed_config(app_name),ExampleMigrator).await.unwrap()
    }
}
