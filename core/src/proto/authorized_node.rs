use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Streaming};

tonic::include_proto!("caretta_sync.authorized_node");
pub struct AuthorizedNodeService {}

#[tonic::async_trait]
impl authorized_node_server::AuthorizedNode for AuthorizedNodeService {
    type ListStream = Pin<Box<dyn Stream<Item = Result<ListResponse, tonic::Status>> + Send>>;

    async fn info(
        &self,
        request: Request<InfoRequest>,
    ) -> Result<Response<InfoResponse>, tonic::Status> {
        todo!()
    }
    async fn list(
        &self,
        request: Request<Streaming<ListRequest>>,
    ) -> Result<Response<Self::ListStream>, tonic::Status> {
        todo!()
    }
}
