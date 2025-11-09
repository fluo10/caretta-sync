use std::{pin::Pin, sync::Arc};

use caretta_sync_core::{
    context::{ServiceContext, ServiceContextExt},
    proto::api::device::{device_service_server::DeviceService, *},
};
use sea_orm::Iden;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Streaming};

use crate::proto_ext::DeviceIdentifierExt;

pub struct DeviceServiceHandler {
    context: Arc<dyn AsRef<ServiceContext> + Send + Sync>,
}

impl DeviceServiceHandler {
    pub fn new(context: &Arc<dyn AsRef<ServiceContext> + Send + Sync>) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

#[tonic::async_trait]
impl DeviceService for DeviceServiceHandler {
    async fn invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, tonic::Status> {
        todo!()
    }
    async fn join(
        &self,
        request: Request<JoinRequest>,
    ) -> Result<Response<JoinResponse>, tonic::Status> {
        todo!()
    }
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, tonic::Status> {
        let request = request.into_inner();
        let public_key = request
            .target
            .ok_or(tonic::Status::invalid_argument(
                "PingRequest.target is required",
            ))?
            .to_public_key()
            .await
            .map_err(|e| tonic::Status::invalid_argument("Invalid public key value"))?
            .ok_or(tonic::Status::not_found("Target device is not found"))?;
        let mut stream = self
            .context
            .as_ref()
            .discover(public_key)
            .await
            .ok_or(tonic::Status::not_found("Target peer address not found"))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(|e| tonic::Status::from_error(Box::new(e)))?;

            let result = iroh_ping::Ping::new()
                .ping(
                    self.context.as_ref().as_endpoint().unwrap(),
                    discovered.into_endpoint_addr(),
                )
                .await
                .map_err(|e| tonic::Status::internal("ping error"))?;
            let response = PingResponse {
                rtt: Some(result.try_into().unwrap()),
            };
            return Ok(Response::new(response));
        }
        Err(tonic::Status::not_found("Not found"))
    }
    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, tonic::Status> {
        todo!()
    }
    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, tonic::Status> {
        todo!()
    }
}
