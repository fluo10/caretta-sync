use std::sync::Arc;

use caretta_sync_core::{context::ServiceContext, proto::api::invitation_token::*};
use tonic::{Request, Response};
pub struct InvitationTokenServiceHandler {
    context: Arc<dyn AsRef<ServiceContext> + Send + Sync>
}

impl InvitationTokenServiceHandler {
    pub fn new(context: &Arc<dyn AsRef<ServiceContext> + Send + Sync>) -> Self {
        Self{context: context.clone()}
    }
}

#[tonic::async_trait]
impl invitation_token_service_server::InvitationTokenService for InvitationTokenServiceHandler {
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