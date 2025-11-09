use caretta_sync_core::{
    config::LogConfig,
    util::{Emptiable, Mergeable},
};
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::parsed_config::{error::ParsedConfigError, types::ParsedLogLevel};

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ParsedLogConfig {
    #[arg(long = "log-level", env = "LOG_LEVEL")]
    pub level: Option<ParsedLogLevel>,
}

impl ParsedLogConfig {
    pub fn default() -> Self {
        Self {
            level: Some(ParsedLogLevel::Info),
        }
    }
}

impl Emptiable for ParsedLogConfig {
    fn empty() -> Self {
        Self { level: None }
    }
    fn is_empty(&self) -> bool {
        self.level.is_none()
    }
}
impl Mergeable for ParsedLogConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.level {
            self.level = Some(x);
        }
    }
}

impl TryFrom<ParsedLogConfig> for LogConfig {
    type Error = ParsedConfigError;
    fn try_from(config: ParsedLogConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            level: config
                .level
                .ok_or(ParsedConfigError::MissingConfig("log.level"))?
                .into(),
        })
    }
}

impl From<LogConfig> for ParsedLogConfig {
    fn from(source: LogConfig) -> Self {
        Self {
            level: Some(source.level.try_into().unwrap()),
        }
    }
}
