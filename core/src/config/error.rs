use http::uri::InvalidUri;
use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("missing config: {0}")]
    MissingConfig(&'static str),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml Deserialization Error")]
    TomlDerialization(#[from] toml::de::Error),
    #[error("Toml Serialization Error")]
    TomlSerialization(#[from] toml::ser::Error),
    #[error("Invalid url: {0}")]
    UriInvalid(#[from] InvalidUri),
    #[error("Db Error: {0}")]
    Db(#[from] DbErr),
}
