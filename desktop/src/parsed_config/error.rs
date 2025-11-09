use std::array::TryFromSliceError;

#[derive(thiserror::Error, Debug)]
pub enum ParsedConfigError {
    #[error("missing config: {0}")]
    MissingConfig(&'static str),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml Deserialization Error")]
    TomlDerialization(#[from] toml::de::Error),
    #[error("Toml Serialization Error")]
    TomlSerialization(#[from] toml::ser::Error),
    #[error("Invalid url: {0}")]
    UriInvalid(#[from] url::ParseError),
    #[error("Failed to get config dir")]
    ConfigDir,
    #[error("Invalid log level: {0}")]
    LogLevel(#[from] crate::parsed_config::types::LogLevelParseError),
    #[error(transparent)]
    SliceTryFrom(#[from] TryFromSliceError),
}
