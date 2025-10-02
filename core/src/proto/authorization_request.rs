use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Streaming};

tonic::include_proto!("caretta_sync.authorization_request");
pub struct AuthorizationRequestService {}

#[tonic::async_trait]
impl authorization_request_server::AuthorizationRequest for AuthorizationRequestService {
    type ListStream = Pin<Box<dyn Stream<Item = Result<ListResponse, tonic::Status>> + Send>>;
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, tonic::Status> {
        todo!()
    }
    async fn accept(&self, request: Request<AcceptRequest>) -> Result<Response<AcceptResponse>, tonic::Status>{
        todo!()
    }
    async fn reject(&self, request: Request<RejectRequest>) -> Result<Response<RejectResponse>, tonic::Status>{
        todo!()
    }
    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoResponse>, tonic::Status>{
        todo!()
    }
    async fn list(&self, request: Request<Streaming<ListRequest>>) -> Result<Response<Self::ListStream>, tonic::Status> {
        todo!()
    }
}