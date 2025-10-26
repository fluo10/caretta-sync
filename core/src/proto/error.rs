#[derive(thiserror::Error, Debug)]
pub enum ProtoSerializeError {
    #[error("Duration parse error: {0}")]
    Duration(#[from] prost_types::DurationError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProtoDeserializeError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("Public key parsing error: {0}")]
    PublicKeyParsing(#[from] iroh::KeyParsingError),
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] std::array::TryFromSliceError),
    #[error("Int parse error: {0}")]
    IntTryFrom(#[from] std::num::TryFromIntError),
    #[error("Unspecified enum: {0}")]
    EnumUnspecified(&'static str)
}
