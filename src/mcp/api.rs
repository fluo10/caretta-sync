use crate::mcp::model::{DevicePingRequest, DevicePingResponse};

#[async_trait::async_trait]
pub trait Api {
    type Error;
    async fn device_ping(
        &self,
        params: DevicePingRequest,
    ) -> Result<DevicePingResponse, Self::Error>;
}
