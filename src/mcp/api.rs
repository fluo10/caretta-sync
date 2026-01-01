#[cfg(feature = "devtools")]
use crate::mcp::model::{DevPingRequest, DevResetRequest, DevResetResponse, DevPingResponse};

#[async_trait::async_trait]
pub trait Api {
    type Error;
    #[cfg(feature = "devtools")]
    async fn dev_ping(
        &self,
        params: DevPingRequest,
    ) -> Result<DevPingResponse, Self::Error>;
}
