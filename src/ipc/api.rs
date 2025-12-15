use std::time::Duration;

#[cfg(feature = "service")]
use rmcp::handler::server::tool::ToolRouter;

use crate::{ipc::{DeviceIdentifier, DeviceInfo }, types::Bytes};

/// A trait for 
#[async_trait::async_trait]
pub trait IpcApi: Sized{
    type Error;
    /// Get device information
    async fn device_get(&self, target: DeviceIdentifier) -> Result<DeviceInfo, Self::Error>;
    
    /// List device information
    async fn device_list(&self) -> Result<Vec<DeviceInfo>, Self::Error>;
    
    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Self::Error>;
    
    /// Remove target device from authorized device table.
    async fn device_remove(&self, target: DeviceIdentifier) -> Result<(), Self::Error>;

    /// Initialize empty user data.
    async fn init(&self) -> Result<(), Self::Error>;
    
    /// Create new token
    async fn invite(&self) -> Result<Bytes, Self::Error>;

    /// Join existing cluster and import its user data
    async fn join(&self, token: Bytes) -> Result<(), Self::Error>;

    #[cfg(feature = "engine")]
    fn tool_router() -> rmcp::handler::server::tool::ToolRouter<Self> ;
}