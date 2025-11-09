use std::{fmt::Display, str::FromStr};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParsedLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl ValueEnum for ParsedLogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Error,
            Self::Warn,
            Self::Info,
            Self::Debug,
            Self::Trace,
        ]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;

        Some(PossibleValue::new(self.as_str()))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid log level")]
pub struct LogLevelParseError;

impl ParsedLogLevel {
    const ERROR: &str = "error";
    const WARN: &str = "warn";
    const INFO: &str = "info";
    const DEBUG: &str = "debug";
    const TRACE: &str = "trace";

    const fn as_str(&self) -> &'static str {
        match self {
            ParsedLogLevel::Error => Self::ERROR,
            ParsedLogLevel::Warn => Self::WARN,
            ParsedLogLevel::Info => Self::INFO,
            ParsedLogLevel::Debug => Self::DEBUG,
            ParsedLogLevel::Trace => Self::TRACE,
        }
    }
}

impl From<ParsedLogLevel> for tracing::Level {
    fn from(value: ParsedLogLevel) -> Self {
        match value {
            ParsedLogLevel::Error => Self::ERROR,
            ParsedLogLevel::Warn => Self::WARN,
            ParsedLogLevel::Info => Self::INFO,
            ParsedLogLevel::Debug => Self::DEBUG,
            ParsedLogLevel::Trace => Self::TRACE,
        }
    }
}
impl TryFrom<tracing::Level> for ParsedLogLevel {
    type Error = LogLevelParseError;
    fn try_from(value: tracing::Level) -> Result<Self, LogLevelParseError> {
        use tracing::Level;
        match value {
            Level::DEBUG => Ok(Self::Debug),
            Level::ERROR => Ok(Self::Error),
            Level::INFO => Ok(Self::Info),
            Level::TRACE => Ok(Self::Trace),
            Level::WARN => Ok(Self::Warn),
            _ => Err(LogLevelParseError),
        }
    }
}

impl Display for ParsedLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ParsedLogLevel {
    type Err = LogLevelParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::ERROR => Ok(Self::Error),
            Self::WARN => Ok(Self::Warn),
            Self::INFO => Ok(Self::Info),
            Self::DEBUG => Ok(Self::Debug),
            Self::TRACE => Ok(Self::Trace),
            _ => Err(LogLevelParseError),
        }
    }
}
