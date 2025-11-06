use std::{marker::PhantomData, path::PathBuf};

use crate::{
    config::{LogConfig, ParsedConfig}, utils::{emptiable::Emptiable, mergeable::Mergeable}
};
use clap::Args;


/// An arguments about config.
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
    ///
    /// This function returns a merged [`ParsedConfig`] from the following two sources (The latter has priority).
    /// 
    /// - Read from the configuration file.
    /// - Specified via arguments or environment variables
    pub fn into_parsed_config(self, app_name: &'static str) -> ParsedConfig {
        let mut config = match self.file_path {
            Some(x) => ParsedConfig::read_or_create_from_path(x).unwrap(),
            None => ParsedConfig::read_or_create(app_name).unwrap()
        };
        config.merge(self.args);
        config
    }
}
