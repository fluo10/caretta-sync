use std::{sync::Arc, time::Duration};

use iroh::{Endpoint, discovery::Discovery as _};
use iroh_docs::api::DocsApi;
use tokio_stream::StreamExt;
use rmcp::{ErrorData, Json, handler::server::wrapper::Parameters, model::{ServerCapabilities, ServerInfo}, tool, tool_router};

use crate::{mcp::{context::Context, model::{DeviceGetRequest, DeviceGetResponse, DeviceIdentifier, DeviceInfo, DeviceListRequest, DeviceListResponse, DevicePingRequest, DevicePingResponse, Error}}, types::{Bytes, Database}};

/// Get device information
pub async fn device_get(ctx: &'static Context, params: Parameters<DeviceGetRequest>) -> Result<Json<DeviceGetResponse>, ErrorData> {
    todo!()
}

/// List device information
pub async fn device_list(ctx: &'static Context, params: Parameters<DeviceListRequest>) -> Result<Vec<DeviceListResponse>, Error> {
    todo!()
}

/// Ping device.
/// 
/// This function is for connectivity test so it's works between non-authorized devices.
pub async fn device_ping(ctx: &'static Context, params: Parameters<DevicePingRequest>) -> Result<Json<DevicePingResponse>, ErrorData> {
    let target = params.0.target;
    let public_key = target
        .to_public_key(&ctx.database)
        .await?
        .ok_or(Error::DeviceNotFound(target.clone()))?;
    let mut stream = ctx.iroh_endpoint.discovery().resolve(public_key.into_inner())
        .ok_or(Error::DeviceNotFound(target))?;
    if let Some(x) = stream.next().await {
        let discovered = x.map_err(Error::from)?;
        match iroh_ping::Ping::new()
            .ping(
                &ctx.iroh_endpoint,
                discovered.into_endpoint_addr(),
            )
            .await {
            Ok(x) => Ok(rmcp::Json(DevicePingResponse{rtt: x})),
            Err(e) => Err(Error::DevicePingFailed(format!("{:?}", e)).into()),
        }
    } else {
        unreachable!()
    }
} 

/// Remove target device from authorized device table.
async fn device_remove(ctx: &'static Context, target: DeviceIdentifier) -> Result<(), Error> {
    todo!()
}
/// Create iroh-docs tichet of user data
async fn device_invite(ctx: &'static Context) -> Result<Bytes, Error> {
    todo!()
}

/// Join exist cluster and import its user data
async fn device_join(ctx: &'static Context, token: Bytes) -> Result<(), Error> {
    todo!()
}

/// Initialize empty user data.
async fn device_init(ctx: &'static Context) -> Result<(), Error> {
    todo!()
}
