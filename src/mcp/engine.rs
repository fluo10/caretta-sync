use std::time::Duration;

use rmcp::{ErrorData, Json, handler::server::wrapper::Parameters, model::{ServerCapabilities, ServerInfo}, tool, tool_router};

use crate::{error::Error, mcp::{DeviceIdentifier, DeviceInfo, DevicePingRequest, DevicePingResponse}, types::Bytes};

#[derive(Clone)]
pub struct IpcEngine {
    
}

#[tool_router]
impl IpcEngine {

    /// Get device information
    #[tool(description = "Get device information")]
    pub async fn device_get(&self, params: Parameters<DeviceIdentifier>) -> Result<Json<DeviceInfo>, ErrorData> {
        todo!()
    }
    
    /// List device information
    async fn device_list(&self) -> Result<Vec<DeviceInfo>, Error> {
        todo!()
    }
    
    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    #[tool(description = "Ping to remote device")]
    async fn device_ping(&self, target: Parameters<DevicePingRequest>) -> Result<Json<DevicePingResponse>, ErrorData> {
        todo!()
    } 

    /// Remove target device from authorized device table.
    async fn device_remove(&self, target: DeviceIdentifier) -> Result<(), Error> {
        todo!()
    }
    /// Create iroh-docs tichet of user data
    async fn device_invite(&self) -> Result<Bytes, Error> {
        todo!()
    }

    /// Join exist cluster and import its user data
    async fn device_join(&self, token: Bytes) -> Result<(), Error> {
        todo!()
    }

    /// Initialize empty user data.
    async fn device_init(&self) -> Result<(), Error> {
        todo!()
    }
    

}

impl rmcp::ServerHandler for IpcEngine {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A device and user manager for data syncronization via iroh p2p".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}