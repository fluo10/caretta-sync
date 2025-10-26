use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Streaming};

tonic::include_proto!("caretta_sync.api.device_verification");
pub struct DeviceVerificationServier;

#[tonic::async_trait]
impl device_verification_service_server::DeviceVerificationService for DeviceVerificationServier {
    async fn request(
        &self,
        request: Request<RequestRequest>,
    ) -> Result<Response<RequestResponse>, tonic::Status> {
        todo!()
    }
    async fn confirm(
        &self,
        request: Request<ConfirmRequest>,
    ) -> Result<Response<ConfirmResponse>, tonic::Status> {
        todo!()
    }
    async fn reject(
        &self,
        request: Request<RejectRequest>,
    ) -> Result<Response<RejectResponse>, tonic::Status> {
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
