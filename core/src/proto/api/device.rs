use std::{pin::Pin, sync::Arc};

use futures::{Stream, StreamExt};
use iroh::{EndpointId, PublicKey};
use sea_orm::Iden;
use tonic::{Request, Response, Streaming};

use crate::{context::ServerContext, error::Error, proto::ProtoDeserializeError};

tonic::include_proto!("caretta_sync.api.device");
pub struct DeviceServer{
    context: Arc<ServerContext>
}

impl DeviceServer {
    pub fn new(context: Arc<ServerContext>) -> Self {
        Self{context}
    }
}

impl Identifier {
    pub async fn to_public_key(&self) -> Result<Option<PublicKey>, ProtoDeserializeError>  {
        use identifier::Value;
        if let Some(x) = self.value.as_ref() {
            match x {
                Value::Id(y) => todo!(),
                Value::Name(y) => todo!(),
                Value::PublicKey(y) => Ok(Some((y).try_into()?))
            }
        } else {
            Err(ProtoDeserializeError::MissingField("value"))
        }
    }
}

#[tonic::async_trait]
impl device_service_server::DeviceService for DeviceServer {
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
        let public_key = request.target.ok_or(tonic::Status::invalid_argument("PingRequest.target is required"))?
            .to_public_key().await
            .map_err(|e| tonic::Status::invalid_argument("Invalid public key value"))?
            .ok_or(tonic::Status::not_found("Target device is not found"))?;
        let mut stream = self.context.discover(public_key).await.ok_or(tonic::Status::not_found("Target peer address not found"))?;
        while let Some(x) = stream.next().await {
            let discovered = x.map_err(|_| tonic::Status::internal("TODO"))?;
            
            let result = iroh_ping::Ping::new().ping(self.context.as_endpoint().unwrap(), discovered.into_endpoint_addr()).await.map_err(|e|tonic::Status::internal("TODO"))?;
            let response = PingResponse{
                rtt: Some(result.try_into().unwrap())
            };
            return Ok(Response::new(response))
            
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