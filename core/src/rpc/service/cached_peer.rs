use crate::{cache::entity::{CachedAddressEntity, CachedPeerEntity, CachedPeerModel}, global::{CACHE_DATABASE_CONNECTION, DATA_DATABASE_CONNECTION}, proto::CachedAddressMessage};
use futures::future::join_all;
use tonic::{Request, Response, Status};

use crate::proto::{cached_peer_service_server::{CachedPeerService, CachedPeerServiceServer}, CachedPeerListRequest, CachedPeerListResponse, CachedPeerMessage};
use sea_orm::prelude::*;

#[derive(Debug, Default)]
pub struct CachedPeerServer {}



#[tonic::async_trait]
impl CachedPeerService for CachedPeerServer {
    async fn list(&self, request: Request<CachedPeerListRequest>) -> Result<Response<CachedPeerListResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let reply = CachedPeerListResponse { 
            peers: join_all( CachedPeerEntity::find().all(CACHE_DATABASE_CONNECTION.get()).await.or_else(|e| Err(Status::from_error(Box::new(e))))?.iter().map(|x| async move {
                let addresses = CachedAddressEntity::find()
                    .all(CACHE_DATABASE_CONNECTION.get())
                    .await
                    .or_else(|e| Err(Status::from_error(Box::new(e))))?;
                Ok::<CachedPeerMessage, Status>(CachedPeerMessage::from((x, &addresses)))
            })).await.into_iter().collect::<Result<Vec<_>,_>>()?,
        };

        Ok(Response::new(reply))
    }
}
