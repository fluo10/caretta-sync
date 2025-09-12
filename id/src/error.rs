#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Outside of range: {0}")]
    OutsideOfRange(u16),
    #[error("Invalid chunk: {0}")]
    InvalidChunk(String),
}

