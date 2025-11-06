use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{config::LogConfig, parsed_config::{error::ParsedConfigError, types::ParsedLogLevel}, utils::{emptiable::Emptiable, mergeable::Mergeable}};

#[derive(Args, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ParsedLogConfig {
    #[arg(long="log-level", env="LOG_LEVEL")]
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

impl Mergeable for Option<ParsedLogConfig> {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.take() {
            if let Some(y) = self.as_mut() {
                y.merge(x);
            } else {
                let _ = self.insert(x);
            }
        };
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