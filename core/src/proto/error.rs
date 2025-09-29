#[derive(thiserror::Error, Debug)]
pub enum ProtoSerializeError {
    #[error("Duration parse error: {0}")]
    Duration(#[from] prost_types::DurationError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProtoDeserializeError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("Signature error: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] std::array::TryFromSliceError),
    #[error("Int parse error: {0}")]
    IntTryFrom(#[from] std::num::TryFromIntError),
}