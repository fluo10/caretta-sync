use std::{array::TryFromSliceError, ffi::OsString};
use tonic::Status;

use caretta_sync_core::proto::ProtoDeserializeError;

#[derive(thiserror::Error, Debug)]
pub enum BackendError {
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
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] TryFromSliceError),
    #[error("protobuf serialization error: {0}")]
    ProtoSerialize(#[from] caretta_sync_core::proto::ProtoSerializeError),
    #[error("protobuf deserialization error: {0}")]
    ProtoDeserialize(#[from] caretta_sync_core::proto::ProtoDeserializeError),
    #[error("Local record error: {0}")]
    LocalDb(#[from] sea_orm::DbErr),
    #[error("Tripod id error: {0}")]
    Mtid(#[from] mtid::Error),
    #[error("Tonic transport error: {0}")]
    TonicTransport(#[from] tonic::transport::Error),
}
