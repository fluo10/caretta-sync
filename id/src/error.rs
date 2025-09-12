#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Outside of range: {0}")]
    OutsideOfRange(u64),
    #[error("Invalid chunk: {0}")]
    InvalidChunk(String),
    #[error("Invalid delemeter: {0}")]
    InvalidDelimiter(char),
    #[error("Invalid length: {0}")]
    InvalidLength(String)
}

