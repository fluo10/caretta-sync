#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("missing config: {0}")]
    MissingConfig(String),
}