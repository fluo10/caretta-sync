
use crate::{cache::entity::CachedPeerModel, error::Error};

tonic::include_proto!("caretta");

pub trait CachedPeerInterface {
    async fn list( req: CachedPeerListRequest) -> Result<CachedPeerListResponse, Error>;
}

