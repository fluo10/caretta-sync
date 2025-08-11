use crate::{cache::entity::{CachedAddressEntity, CachedPeerEntity, CachedPeerModel}, global::{CACHE_DATABASE_CONNECTION, DATA_DATABASE_CONNECTION}, rpc::proto::CachedAddressMessage};
use tonic::{Request, Response, Status};

use crate::rpc::proto::{cached_peer_service_server::{CachedPeerService, CachedPeerServiceServer}, CachedPeerListRequest, CachedPeerListResponse, CachedPeerMessage};
use sea_orm::prelude::*;

#[derive(Debug, Default)]
pub struct CachedPeerServer {}



#[tonic::async_trait]
impl CachedPeerService for CachedPeerServer {
    async fn list(&self, request: Request<CachedPeerListRequest>) -> Result<Response<CachedPeerListResponse>, Status> {
        println!("Got a request: {:?}", request);
            todo!();
        
        let reply = CachedPeerListResponse { 
            peers: CachedPeerEntity::find().all(CACHE_DATABASE_CONNECTION.get()).await.or_else(|e| Err(Status::from_error(Box::new(e))))?.into_iter().map(|x| {
                let addresses = CachedAddressEntity::find().all(CACHE_DATABASE_CONNECTION.get()).await.or_else(|e| Err(Status::from_error(Box::new(e))))?
                    .map(|x| {
                        CachedAddressMessage::from(x)
                    });
                CachedPeerMessage::from(x)
            }).collect(),
        };

        Ok(Response::new(reply))
    }
}
