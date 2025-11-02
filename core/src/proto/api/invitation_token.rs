use std::sync::Arc;

use tonic::{Request, Response};

use crate::context::ServerContext;

tonic::include_proto!("caretta_sync.api.invitation_token");

pub struct InvitationTokenServer {
    context: Arc<ServerContext>
}

impl InvitationTokenServer {
    pub fn new(context: Arc<ServerContext>) -> Self {
        Self{context}
    }
}

#[tonic::async_trait]
impl invitation_token_service_server::InvitationTokenService for InvitationTokenServer {
    async fn revoke(
        &self,
        request: Request<RevokeRequest>,
    ) -> Result<Response<RevokeResponse>, tonic::Status> {
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