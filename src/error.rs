use std::{array::TryFromSliceError, ffi::OsString};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature="engine")]
    #[error("Iroh bind error: {0}")]
    IrohBind(#[from] iroh::endpoint::BindError),
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(&'static str),
    #[error("Parse OsString error: {0:?}")]
    OsStringConvert(std::ffi::OsString),
    #[error("slice parse error: {0}")]
    SliceTryFrom(#[from] TryFromSliceError),
    #[error("Caretta id error: {0}")]
    CarettaId(#[from] caretta_id::Error),
}

impl From<std::ffi::OsString> for Error {
    fn from(s: OsString) -> Error {
        Self::OsStringConvert(s)
    }
}
