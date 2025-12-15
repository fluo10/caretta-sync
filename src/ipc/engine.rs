use std::time::Duration;

use rmcp::{handler::server::wrapper::Parameters, model::{ServerCapabilities, ServerInfo}, tool, tool_router};

use crate::{error::Error, ipc::{DeviceIdentifier, DeviceInfo, IpcApi}, types::Bytes};

#[derive(Clone)]
pub struct IpcEngine {
    
}

#[tool_router]
impl IpcEngine {

    /// Get device information
    #[tool(description = "Get device information")]
    async fn device_get(&self, params: Parameters<DeviceIdentifier>) -> Result<DeviceInfo, Error> {
        todo!()
    }
    
    /// List device information
    async fn device_list(&self) -> Result<Vec<DeviceInfo>, Error> {
        todo!()
    }
    
    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Error> {
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
            instructions: Some("A device and user manager for data syncronization via p2p".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}