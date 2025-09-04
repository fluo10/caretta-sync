use std::pin::Pin;

use iroh::{Endpoint, NodeId};
use tonic::{Response, Request, Status};
use tokio_stream::Stream;

use crate::proto::{RemoteInfoIterRequest, RemoteInfoMessage, RemoteInfoRequest, RemoteInfoResponse};

#[tonic::async_trait]
pub trait AsEndpoint: Send + Sync + 'static {
    async fn as_endpoint(&self) -> &Endpoint;
}
pub struct IrohServer<T>
where 
    T: AsEndpoint
{
    endpoint: T
}

#[tonic::async_trait]
impl<T> crate::proto::iroh_server::Iroh for IrohServer<T> 
where 
    T: AsEndpoint
{
    type RemoteInfoIterStream = Pin<Box<dyn Stream<Item = Result<RemoteInfoResponse, Status>> + Send>>;
    async fn remote_info(&self, request: Request<RemoteInfoRequest>) -> Result<Response<RemoteInfoResponse>, Status> {
        let node_id = NodeId::try_from(request.into_inner()).or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        let remote_info: Option<RemoteInfoMessage> = self.endpoint.as_endpoint().await.remote_info(node_id).map(|x| x.try_into()).transpose().or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        Ok(Response::new(RemoteInfoResponse::from(remote_info)))
    } 
    async fn remote_info_iter(&self, _: Request<RemoteInfoIterRequest>) -> Result<Response<Self::RemoteInfoIterStream>, Status> {
        let iter = self.endpoint.as_endpoint().await.remote_info_iter();
        let stream = futures::stream::iter(iter);
        todo!()
    }
}