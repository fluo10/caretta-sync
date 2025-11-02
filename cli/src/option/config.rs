use std::{marker::PhantomData, path::PathBuf};

use caretta_sync_core::{
    config::{LogConfig, ParsedConfig}, context::{ClientContext, ServerContext}, utils::{emptiable::Emptiable, mergeable::Mergeable}
};
use clap::Args;

use sea_orm_migration::MigratorTrait;
use tokio::sync::OnceCell;

#[derive(Args, Clone, Debug)]
pub struct ConfigOptionArgs
{
    #[arg(short = 'c', long = "config_path", global = true, env = "CONFIG_PATH")]
    pub file_path: Option<PathBuf>,
    #[command(flatten)]
    pub args: ParsedConfig,
}

impl ConfigOptionArgs
{
    pub fn into_parsed_config(self, app_name: &'static str) -> ParsedConfig {
        let mut config = ParsedConfig::default(app_name);
        config.merge(match self.file_path {
            Some(x) => ParsedConfig::read_or_create_from_path(x).unwrap(),
            None => ParsedConfig::read_or_create(app_name).unwrap()
        });
        config.merge(self.args);
        config
    }

    pub fn into_client_context(self, app_name: &'static str) -> ClientContext {
        ClientContext::from_parsed_config(self.into_parsed_config(app_name)).unwrap()
    }
    pub async fn into_server_context<M>(self, app_name: &'static str, migrator: PhantomData<M>) -> ServerContext
    where 
        M: MigratorTrait
    {
        ServerContext::from_parsed_config(self.into_parsed_config(app_name),migrator).await.unwrap()
    }
}
