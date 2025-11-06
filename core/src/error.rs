use std::{array::TryFromSliceError, ffi::OsString};
use tonic::Status;

use crate::proto::ProtoDeserializeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config error: {0}")]
    Config(#[from] crate::config::ConfigError),
    #[error("Infallible: {0}")]
    Infallible(#[from] std::convert::Infallible),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Iroh bind error: {0}")]
    IrohBind(#[from] iroh::endpoint::BindError),
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(&'static str),
    #[error("Parse OsString error: {0:?}")]
    OsStringConvert(std::ffi::OsString),
    #[cfg(feature = "cli")]
    #[error("Parse args error: {0}")]
    ParseCommand(#[from] clap::Error),
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] TryFromSliceError),
    #[error("toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("protobuf serialization error: {0}")]
    ProtoSerialize(#[from] crate::proto::ProtoSerializeError),
    #[error("protobuf deserialization error: {0}")]
    ProtoDeserialize(#[from] crate::proto::ProtoDeserializeError),
    #[error("Local record error: {0}")]
    LocalDb(#[from] sea_orm::DbErr),
    #[error("Tripod id error: {0}")]
    Mtid(#[from] mtid::Error),
    #[error("Tonic transport error: {0}")]
    TonicTransport(#[from] tonic::transport::Error)
}

impl From<std::ffi::OsString> for Error {
    fn from(s: OsString) -> Error {
        Self::OsStringConvert(s)
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::ProtoDeserialize(x) => match x {
                ProtoDeserializeError::MissingField(x) => {
                    Self::invalid_argument(format!("{} is required", x))
                }
                _ => Status::unimplemented("Unimplemented protobuf deserialize error status"),
            },
            _ => Status::unimplemented("Unimplemented error status"),
        }
    }
}
