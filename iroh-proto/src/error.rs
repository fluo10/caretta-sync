#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duration parse error: {0}")]
    Duration(#[from] prost_types::DurationError),
    #[error("Signature error: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] std::array::TryFromSliceError)
}
