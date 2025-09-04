use std::pin::Pin;

use iroh::{endpoint, Endpoint, NodeId};
use tonic::{Response, Request, Status};
use tokio_stream::Stream;

use crate::proto::{RemoteInfoIterRequest, RemoteInfoMessage, RemoteInfoRequest, RemoteInfoResponse};

pub struct IrohServer
where 
{
    endpoint: Endpoint
}

impl From<Endpoint> for IrohServer {
    fn from(endpoint: Endpoint) -> Self {
        Self{endpoint: endpoint}
    }
}

#[tonic::async_trait]
impl crate::proto::iroh_server::Iroh for IrohServer {
    type RemoteInfoIterStream = Pin<Box<dyn Stream<Item = Result<RemoteInfoResponse, Status>> + Send>>;
    async fn remote_info(&self, request: Request<RemoteInfoRequest>) -> Result<Response<RemoteInfoResponse>, Status> {
        let node_id = NodeId::try_from(request.into_inner()).or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        let remote_info: Option<RemoteInfoMessage> = self.endpoint.remote_info(node_id).map(|x| x.try_into()).transpose().or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        Ok(Response::new(RemoteInfoResponse::from(remote_info)))
    } 
    async fn remote_info_iter(&self, _: Request<RemoteInfoIterRequest>) 
        -> Result<Response<Self::RemoteInfoIterStream>, Status> {
        let iter = self.endpoint.remote_info_iter()
            .map(|x| {
                RemoteInfoMessage::try_from(x).map(|x| RemoteInfoResponse::from(x)).or_else(|e| {
                    Err(Status::from_error(Box::new(e)))
                })
            });
        let stream = futures::stream::iter(iter);
        Ok(Response::new(Box::pin(stream)))
    }
}