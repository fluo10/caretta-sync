use iroh::KeyParsingError;

#[derive(Debug, thiserror::Error)]
pub enum InvitationTokenDeserializeError {
    #[error(transparent)]
    EndpointIdParsing(#[from] KeyParsingError),
    #[error("Invalid token id: {0}")]
    TokenIdOversized(#[from] mtid::Error),
    #[error("Invalid date time value.")]
    DateTimeInvalid,
}
