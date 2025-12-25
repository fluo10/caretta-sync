use caretta_framework::mcp::{Api as _, ServiceContext, model::*};
use rmcp::{
    ErrorData, Json,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{Meta, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

#[derive(Clone, Debug)]
pub struct Service {
    pub context: &'static ServiceContext,
    pub tool_router: ToolRouter<Service>,
}

#[tool_router]
impl Service {
    pub fn new(context: &'static ServiceContext) -> Self {
        Self {
            context,
            tool_router: Self::tool_router(),
        }
    }
    /// Ping device.
    ///
    /// This function is for connectivity test so it's works between non-authorized devices.
    #[tool(description = "Ping to remote device")]
    async fn device_ping(
        &self,
        params: Parameters<DevicePingRequest>,
    ) -> Result<Json<DevicePingResponse>, ErrorData> {
        self.context
            .device_ping(params.0)
            .await
            .map(|x| Json(x))
            .map_err(Into::<ErrorData>::into)
    }
}
#[tool_handler(meta = Meta(rmcp::object!({"tool_meta_key": "tool_meta_value"})))]
impl rmcp::ServerHandler for Service {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "A device and user manager for data syncronization via iroh p2p".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl From<&'static ServiceContext> for Service {
    fn from(value: &'static ServiceContext) -> Self {
        Self::new(value)
    }
}
