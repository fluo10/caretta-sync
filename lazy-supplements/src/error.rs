#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
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
    #[error("Parse args error: {0}")]
    ParseCommand(#[from] clap::Error),
    #[error("Readline error: {0}")]
    Readline(#[from] rustyline::error::ReadlineError),
    #[error("Shell word split error: {0}")]
    ShellWord(#[from] shell_words::ParseError),
    #[error("toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error), 
    #[error("Transport error: {0}")]
    Transport(#[from]libp2p::TransportError<std::io::Error>)
}