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
    #[arg(short = 'c', long = "config", global = true, env = "CONFIG")]
    pub file_path: Option<PathBuf>,
    #[command(flatten)]
    pub args: ParsedConfig,
}

impl ConfigOptionArgs
{
    /// Convert [`ConfigOptionArgs`] into [`ParsedConfig`]
    pub fn into_parsed_config(self, app_name: &'static str) -> ParsedConfig {
        let mut config = match self.file_path {
            Some(x) => ParsedConfig::read_or_create_from_path(x).unwrap(),
            None => ParsedConfig::read_or_create(app_name).unwrap()
        };
        config.merge(self.args);
        config
    }
}
