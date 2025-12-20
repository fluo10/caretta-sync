use caretta_sync::mcp::{Service as SyncService, *};
use rmcp::{model::{ServerCapabilities, ServerInfo}, tool, tool_router};

#[derive(Clone, Debug)]
pub struct Service {
    pub sync_service: SyncService
}

#[tool_router]
impl Service {

    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    #[tool(description = "Ping to remote device")]
    async fn device_ping(&self, params: Parameters<DevicePingRequest>) -> Result<Json<DevicePingResponse>, ErrorData> {
        self.sync_service.device_ping(params)
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

impl rmcp::ServerHandler for Service {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A device and user manager for data syncronization via iroh p2p".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}