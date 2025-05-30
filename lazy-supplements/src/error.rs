#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB Error: {0}")]
    Db(#[from]sea_orm::DbErr),
    #[error("IO Error: {0}")]
    Io(#[from]std::io::Error),
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(String),
    #[error("toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error), 
}