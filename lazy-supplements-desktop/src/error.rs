pub use lazy_supplements_core::error::Error as CoreError;

#[derive(thiserror::Error, Debug)]
pub enum DesktopError {
    #[error("Parse args error: {0}")]
    ParseCommand(#[from] clap::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Core(#[from] CoreError),
    #[error("{0}")]
    Desktop(#[from] DesktopError),
}

