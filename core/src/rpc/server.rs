use tonic::transport::Server;

use crate::{proto::cached_peer_service_server::CachedPeerServiceServer, rpc::service::cached_peer::CachedPeerService};


pub async fn start_server() ->Result<(), Error> {
    let addr = "[::1]:50051".parse()?;
    let cached_peer_server = CachedPeerService::default();
    Server::builder()
        .add_service(CachedPeerServiceServer::new(cached_peer_server))
        .serve(addr)
        .await?;
    
    Ok(())

}