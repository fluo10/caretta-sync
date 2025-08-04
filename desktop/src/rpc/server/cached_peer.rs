use caretta_core::{cache::entity::CachedPeerEntity, global::DATA_DATABASE_CONNECTION};
use tonic::{Request, Response, Status};

use crate::rpc::proto::{cached_peer_service_server::{CachedPeerService, CachedPeerServiceServer}, CachedPeerListRequest, CachedPeerListResponse};
use sea_orm::prelude::*;

#[derive(Debug, Default)]
pub struct CachedPeerServer {}

#[tonic::async_trait]
impl CachedPeerService for CachedPeerServer {
    async fn list(&self, request: Request<CachedPeerListRequest>) -> Result<Response<CachedPeerListResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let reply = CachedPeerListResponse { 
            peers: CachedPeerEntity::find().all(DATA_DATABASE_CONNECTION.get()).await?
        };
    }
    
}