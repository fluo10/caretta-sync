use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status, Streaming};

use crate::global::IROH_ENDPOINT;

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
