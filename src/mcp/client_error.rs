use rmcp::ServiceError;

/// An error for MCP client
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to deserialize response: {0}")]
    DeserializeResponse(serde_json::Error),
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
}