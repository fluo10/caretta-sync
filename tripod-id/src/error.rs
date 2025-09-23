#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("expected under {expected}, found {found}")]
    OutsideOfRange{
        expected: u64,
        found: u64,
    },
    #[error("Invalid chunk: {0}")]
    InvalidChunk(String),
    #[error("Length of id expected {} or {} but found {found}: {raw}", .expected.0, .expected.1 )]
    InvalidLength{
        expected: (u8, u8),
        found: usize,
        raw: String
    },
    #[error("Number of chunks expected {expected} but  {found}: {raw}")]
    InvalidLengthOfChunks{
        expected: u8,
        found: usize,
        raw: String,
    },
    #[error("Delimiter expected '-' or '_' but '{found:?}' found: {raw}")]
    InvalidDelimiter{
        found: Vec<char>,
        raw: String,
    }
}

