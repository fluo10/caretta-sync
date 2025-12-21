use rmcp::ServiceError;

use crate::mcp::model::{DevicePingRequest, DevicePingResponse};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Failed to deserialize response: {0}")]
    DeserializeResponse(serde_json::Error),
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
}

#[async_trait::async_trait]
pub trait Api {
    async fn device_ping(&self, params: DevicePingRequest) -> Result<DevicePingResponse, ApiError>;
}
