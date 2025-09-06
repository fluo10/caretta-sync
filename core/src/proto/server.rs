use std::pin::Pin;

use iroh::{endpoint, Endpoint, NodeId};
use tonic::{Response, Request, Status};
use tokio_stream::Stream;

use crate::{global::IROH_ENDPOINT, proto::{error::ProtoDeserializeError, RemoteInfoIterRequest, RemoteInfoMessage, RemoteInfoRequest, RemoteInfoResponse}};

pub struct CarettaSyncServer{}

#[tonic::async_trait]
impl crate::proto::caretta_sync_server::CarettaSync for CarettaSyncServer {
    type RemoteInfoIterStream = Pin<Box<dyn Stream<Item = Result<RemoteInfoResponse, Status>> + Send>>;
    async fn remote_info(&self, request: Request<RemoteInfoRequest>) -> Result<Response<RemoteInfoResponse>, Status> {
        let node_id = NodeId::try_from(request.into_inner().node_id.ok_or(Status::from_error(Box::new(ProtoDeserializeError::MissingField("node_id"))))?).or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        let remote_info: Option<RemoteInfoMessage> = IROH_ENDPOINT.get_unchecked().remote_info(node_id).map(|x| x.try_into()).transpose().or_else(|e| {
            Err(Status::from_error(Box::new(e)))
        })?;
        Ok(Response::new(RemoteInfoResponse::from(remote_info)))
    } 
    async fn remote_info_iter(&self, _: Request<RemoteInfoIterRequest>) 
        -> Result<Response<Self::RemoteInfoIterStream>, Status> {
        let iter = IROH_ENDPOINT.get_unchecked().remote_info_iter()
            .map(|x| {
                RemoteInfoMessage::try_from(x).map(|x| RemoteInfoResponse::from(x)).or_else(|e| {
                    Err(Status::from_error(Box::new(e)))
                })
            });
        let stream = futures::stream::iter(iter);
        Ok(Response::new(Box::pin(stream)))
    }
}