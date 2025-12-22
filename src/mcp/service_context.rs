use std::sync::{Arc, OnceLock};

use iroh::{Endpoint, discovery::Discovery as _};
use iroh_docs::api::DocsApi;
use rmcp::{ErrorData, Json, handler::server::wrapper::Parameters};
use sea_orm::DatabaseConnection;
use tokio_stream::StreamExt as _;

use crate::{
    mcp::{
        Api,
        model::{self, DevicePingRequest, DevicePingResponse},
    },
    types::{AppDatabase, Database},
};

#[derive(Debug)]
pub struct ServiceContext {
    pub app_database: AppDatabase,
    pub database: Database,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}

#[async_trait::async_trait]
impl Api for ServiceContext {
    type Error = ErrorData;
    async fn device_ping(
        &self,
        params: DevicePingRequest,
    ) -> Result<DevicePingResponse, ErrorData> {
        let target = params.target;
        let public_key = target
            .to_public_key(&self.database)
            .await?
            .ok_or(model::Error::DeviceNotFound(target.clone()))?;
        let mut stream = self
            .iroh_endpoint
            .discovery()
            .resolve(public_key.into_inner())
            .ok_or(model::Error::DeviceNotFound(target))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(model::Error::from)?;
            match iroh_ping::Ping::new()
                .ping(&self.iroh_endpoint, discovered.into_endpoint_addr())
                .await
            {
                Ok(x) => Ok(DevicePingResponse { rtt: x }),
                Err(e) => Err(model::Error::DevicePingFailed(format!("{:?}", e)).into()),
            }
        } else {
            unreachable!()
        }
    }
}
