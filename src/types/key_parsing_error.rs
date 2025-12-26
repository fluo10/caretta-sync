
/// Error when deserializing a public key or a secret key.
#[derive(Debug, thiserror::Error)]
pub enum KeyParsingError{
    #[error("Expected base32 string, found {0}")]
    InvalidBase32String(#[from] crate::util::DecodeBase32Error),
    #[error("invalid length {0}")]
    InvalidBytesLength(#[from] std::array::TryFromSliceError),
    #[error("invalid value {0}")]
    InvalidBytesValue(#[from] ed25519_dalek::SignatureError),
}