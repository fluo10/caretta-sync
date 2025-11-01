use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Streaming};

tonic::include_proto!("caretta_sync.api.device");
pub struct DeviceServer;

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
        todo!()
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