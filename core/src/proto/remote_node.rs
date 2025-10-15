use std::{pin::Pin, time::Duration};

use futures::{Stream, future::Remote};
use iroh::{
    PublicKey,
    endpoint::{DirectAddrInfo, RemoteInfo},
};
use mtid::Dtid;
use tonic::{Request, Response, Status, Streaming};

use crate::{
    error::Error,
    global::IROH_ENDPOINT,
    proto::error::{ProtoDeserializeError, ProtoSerializeError},
};

tonic::include_proto!("caretta_sync.remote_node");

pub struct RemoteNodeServer {}

#[tonic::async_trait]
impl remote_node_server::RemoteNode for RemoteNodeServer {
    type ListStream = Pin<Box<dyn Stream<Item = Result<ListResponse, Status>> + Send>>;
    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        todo!()
    }
    async fn list(
        &self,
        request: Request<Streaming<ListRequest>>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let iter = IROH_ENDPOINT.get_unchecked().remote_info_iter().map(|x| {
            todo!();
        });
        let stream = futures::stream::iter(iter);
        Ok(Response::new(Box::pin(stream)))
    }
}
