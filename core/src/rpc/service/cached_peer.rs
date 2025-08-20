use crate::{cache::entity::{CachedAddressEntity, CachedPeerEntity, CachedPeerModel}, global::{DATABASE_CONNECTIONS}, proto::CachedAddressMessage};
use futures::future::join_all;
use tonic::{Request, Response, Status};

use crate::proto::{cached_peer_service_server::{CachedPeerServiceServer}, CachedPeerListRequest, CachedPeerListResponse, CachedPeerMessage};
use sea_orm::prelude::*;

#[derive(Debug, Default)]
pub struct CachedPeerService {}



#[tonic::async_trait]
impl crate::proto::cached_peer_service_server::CachedPeerService for CachedPeerService {
    async fn list(&self, request: Request<CachedPeerListRequest>) -> Result<Response<CachedPeerListResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let reply = CachedPeerListResponse { 
            peers: join_all( CachedPeerEntity::find().all(DATABASE_CONNECTIONS.get_cache_unchecked()).await.or_else(|e| Err(Status::from_error(Box::new(e))))?.iter().map(|x| async move {
                let addresses = CachedAddressEntity::find()
                    .all(DATABASE_CONNECTIONS.get_cache_unchecked())
                    .await
                    .or_else(|e| Err(Status::from_error(Box::new(e))))?;
                Ok::<CachedPeerMessage, Status>(CachedPeerMessage::from((x, &addresses)))
            })).await.into_iter().collect::<Result<Vec<_>,_>>()?,
        };

        Ok(Response::new(reply))
    }
}
