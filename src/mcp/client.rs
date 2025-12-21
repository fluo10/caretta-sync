use rmcp::{RoleClient, ServiceError, ServiceExt as _, model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation, InitializeRequestParam}, service::RunningService, transport::StreamableHttpClientTransport};

use crate::mcp::{Api, ApiError, model::*};

pub struct Client {
    service: RunningService<RoleClient, InitializeRequestParam>
}


impl Client {
    pub  async fn connect(url: &str, client_info: ClientInfo) -> Result<Self, crate::error::Error> {
        let transport = StreamableHttpClientTransport::from_uri(url);
        let client =  client_info.serve(transport).await.inspect_err(|e| {
            tracing::error!("client error: {:?}", e);
        })?;
        

        // Initialize
        let server_info = client.peer_info();
        tracing::info!("Connected to server: {server_info:#?}");

        Ok(Self{
            service: client
        })

    }

    // Close client connection
    pub async fn close(self) -> Result<(), tokio::task::JoinError> {
        self.service.cancel().await?;
        Ok(())

    }
}

#[async_trait::async_trait]
impl Api for  Client {
    
    async fn device_ping(&self, params: DevicePingRequest) -> Result<DevicePingResponse, ApiError> {
        let tool_result = self.service
            .call_tool(CallToolRequestParam {
                name: "increment".into(),
                arguments: serde_json::json!({}).as_object().cloned(),
            })
            .await?;
        tracing::info!("Tool result: {tool_result:#?}");
        let result = tool_result.into_typed().map_err(|e| ApiError::DeserializeResponse(e))?;
        Ok(result)
    }

}