use std::{fmt::Display, str::FromStr};

use crate::utils::{emptiable::Emptiable, mergeable::Mergeable};
#[cfg(feature = "cli")]
use clap::Args;
#[cfg(feature="cli")]
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::config::error::ConfigError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
#[cfg(feature="cli")]
impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Error, Self::Warn, Self::Info, Self::Debug, Self::Trace]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;

        Some(PossibleValue::new(self.as_str()))
    }
}

#[derive(Debug, thiserror::Error,)]
#[error("Invalid log level")]
pub struct LogLevelParseError;

impl LogLevel {
    const ERROR: &str = "error";
    const WARN: &str = "warn";
    const INFO: &str = "info";
    const DEBUG: &str = "debug";
    const TRACE: &str = "trace";

    const fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}
impl TryFrom<tracing::Level> for LogLevel {
    type Error = LogLevelParseError;
    fn try_from(value: tracing::Level) -> Result<Self, LogLevelParseError> {
        use tracing::Level;
        match value {
            Level::DEBUG => Ok(Self::Debug),
            Level::ERROR => Ok(Self::Error),
            Level::INFO => Ok(Self::Info),
            Level::TRACE => Ok(Self::Trace),
            Level::WARN => Ok(Self::Warn),
            _ => Err(LogLevelParseError)
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LogLevel {
    type Err = LogLevelParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::ERROR => Ok(Self::Error),
            Self::WARN => Ok(Self::Warn),
            Self::INFO => Ok(Self::Info),
            Self::DEBUG => Ok(Self::Debug),
            Self::TRACE => Ok(Self::Trace),
            _ => Err(LogLevelParseError)
        }
    }
}




#[derive(Clone, Debug)]
pub struct LogConfig {
    pub level: tracing::Level ,
}

impl TryFrom<PartialLogConfig> for LogConfig {
    type Error = ConfigError;
    fn try_from(config: PartialLogConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            level: config
                .level
                .ok_or(ConfigError::MissingConfig("log.level"))?
                .into(),
        })
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialLogConfig {
    pub level: Option<LogLevel>,
}

impl PartialLogConfig {
    pub fn default(_: &'static str) -> Self {
        Self {
            level: Some(LogLevel::Warn),
        }
    }
}

impl Emptiable for PartialLogConfig {
    fn empty() -> Self {
        Self { level: None }
    }
    fn is_empty(&self) -> bool {
        self.level.is_none()
    }
}

impl From<LogConfig> for PartialLogConfig {
    fn from(source: LogConfig) -> Self {
        Self {
            level: Some(source.level.try_into().unwrap()),
        }
    }
}

impl Mergeable for PartialLogConfig {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.level {
            self.level = Some(x);
        }
    }
}

impl Mergeable for Option<PartialLogConfig> {
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
