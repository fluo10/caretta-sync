use std::{sync::Arc, time::Duration};

use iroh::{Endpoint, discovery::Discovery as _};
use iroh_docs::api::DocsApi;
use tokio_stream::StreamExt;
use rmcp::{ErrorData, Json, handler::server::wrapper::Parameters, model::{ServerCapabilities, ServerInfo}, tool, tool_router};

use crate::{error::Error, mcp::{DeviceIdentifier, DeviceInfo, DevicePingRequest, DevicePingResponse, McpError, ServiceGenerator}, types::{Bytes, Database}};

#[derive(Clone, Debug)]
pub struct Service {
    pub database: Arc<Database>,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}

#[tool_router]
impl Service {

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
    pub async fn device_ping(&self, params: Parameters<DevicePingRequest>) -> Result<Json<DevicePingResponse>, ErrorData> {
        let target = params.0.target;
        let public_key = target
            .to_public_key(&self.database)
            .await?
            .ok_or(McpError::DeviceNotFound(target.clone()))?;
        let mut stream = self.iroh_endpoint.discovery().resolve(public_key.into_inner())
            .ok_or(McpError::DeviceNotFound(target))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(McpError::from)?;
            match iroh_ping::Ping::new()
                .ping(
                    &self.iroh_endpoint,
                    discovered.into_endpoint_addr(),
                )
                .await {
                Ok(x) => Ok(rmcp::Json(DevicePingResponse{rtt: x})),
                Err(e) => Err(McpError::DevicePingFailed(format!("{:?}", e)).into()),
            }
        } else {
            unreachable!()
        }
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

impl From<&'static ServiceGenerator> for Service {
    fn from(value: &'static ServiceGenerator) -> Self {
        Service {
            database:value.database.clone(),
            iroh_endpoint: value.iroh_endpoint.clone(),
            docs: value.docs.clone(),
        }
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