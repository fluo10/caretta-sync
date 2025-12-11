use std::{fmt::Display, str::FromStr};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Default,
    Quiet,
    Verbose,
}

impl ValueEnum for Verbosity {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Default,
            Self::Quiet,
            Self::Verbose,
        ]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;

        Some(PossibleValue::new(self.as_str()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerbosityError {
    #[error("Parse check config mode error")]
    Parse,
}



impl Verbosity {
    const DEFAULT: &str = "default";
    const VERBOSE: &str = "verbose";
    const QUIET: &str = "quiet";

    const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => Self::DEFAULT,
            Self::Verbose => Self::VERBOSE,
            Self::Quiet => Self::QUIET,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Default
    }
}

impl Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Verbosity {
    type Err = VerbosityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::DEFAULT => Ok(Self::Default),
            Self::QUIET => Ok(Self::Quiet),
            Self::VERBOSE => Ok(Self::Verbose),
            _ => Err(VerbosityError::Parse),
        }
    }
}
