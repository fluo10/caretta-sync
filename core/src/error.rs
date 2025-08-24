use std::ffi::OsString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error(transparent)]
    CiborDeserialize(#[from] ciborium::de::Error<std::io::Error>),
    #[error(transparent)]
    CiborSerialize(#[from] ciborium::ser::Error<std::io::Error>),
    #[error("Config error: {0}")]
    Config(#[from] crate::config::error::ConfigError),
    #[error("DB Error: {0}")]
    Db(#[from]sea_orm::DbErr),
    #[error("Dial Error: {0}")]
    Dial(#[from] libp2p::swarm::DialError),
    #[error("Decoding identity error: {0}")]
    IdentityDecoding(#[from] libp2p::identity::DecodingError),
    #[error("Infallible: {0}")]
    Infallible(#[from] std::convert::Infallible),
    #[error("IO Error: {0}")]
    Io(#[from]std::io::Error),
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(&'static str),
    #[error("Multiaddr error: {0}")]
    Multiaddr(#[from] libp2p::multiaddr::Error),
    #[error("Noise error: {0}")]
    Noise(#[from] libp2p::noise::Error),
    #[error("Parse OsString error: {0:?}")]
    OsStringConvert(std::ffi::OsString),
    #[cfg(feature="cli")]
    #[error("Parse args error: {0}")]
    ParseCommand(#[from] clap::Error),
    #[error("toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Transport error: {0}")]
    Transport(#[from]libp2p::TransportError<std::io::Error>)
}

impl From<std::ffi::OsString> for Error {
    fn from(s: OsString) -> Error {
        Self::OsStringConvert(s)
    }
}