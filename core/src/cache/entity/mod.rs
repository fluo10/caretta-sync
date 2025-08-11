mod cached_peer;
mod cached_address;

pub use cached_peer::{
    ActiveModel as CachedPeerActiveModel,
    Column as CachedPeerColumn,
    Model as CachedPeerModel,
    Entity as CachedPeerEntity,
};

pub use cached_address::{
    ActiveModel as CachedAddressActiveModel,
    Column as CachedAddressColumn,
    Model as CachedAddressModel,
    Entity as CachedAddressEntity,
};


#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::{cache::entity::cached_peer, global::get_or_init_test_cache_database};

    use super::*;

    use libp2p::{identity::{self, Keypair}, multiaddr, swarm::handler::multi, Multiaddr, PeerId};
    use sea_orm::ActiveModelTrait;



     #[tokio::test]
    async fn insert() {
        let db = get_or_init_test_cache_database().await;
        let peer_id = Keypair::generate_ed25519().public().to_peer_id();
        let multiaddr = Multiaddr::empty()
            .with(Ipv4Addr::new(127,0,0,1).into())
            .with(multiaddr::Protocol::Tcp(0));
        let inserted_cached_peer: CachedPeerModel = CachedPeerActiveModel::new(peer_id.clone())
                .insert(db).await.unwrap();
        let inserted_cached_address: CachedAddressModel = CachedAddressActiveModel::new(inserted_cached_peer.id, multiaddr.clone())
            .insert(db).await.unwrap();
        assert_eq!(PeerId::from(inserted_cached_peer.peer_id), peer_id);
        assert_eq!(Multiaddr::from(inserted_cached_address.multiaddress), multiaddr);     
    }

}