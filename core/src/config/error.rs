use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("missing config: {0}")]
    MissingConfig(String),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml Deserialization Error")]
    TomlDerialization(#[from] toml::de::Error),
    #[error("Toml Serialization Error")]
    TomlSerialization(#[from] toml::ser::Error),
    #[error("Invalid url: {0}")]
    InvalidUrl(Url)
}