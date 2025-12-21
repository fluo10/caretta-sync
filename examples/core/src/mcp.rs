use caretta_sync::mcp::{context::Context, tool::*, model::*};
use rmcp::{ErrorData, Json, handler::server::{tool::ToolRouter, wrapper::Parameters}, model::{ServerCapabilities, ServerInfo}, tool, tool_router};

#[derive(Clone, Debug)]
pub struct Service {
    pub context: &'static Context,
    pub tool_router: ToolRouter<Service>,
}

#[tool_router]
impl Service {
    pub fn new(context: &'static Context) -> Self {
        Self {
            context,
            tool_router: Self::tool_router(),
        }
    }
    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    #[tool(description = "Ping to remote device")]
    async fn device_ping(&self, params: Parameters<DevicePingRequest>) -> Result<Json<DevicePingResponse>, ErrorData> {
        device_ping(self.context, params).await
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

impl From<&'static Context> for Service {
    fn from(value: &'static Context) -> Self {
        Self::new(value)
    }
}