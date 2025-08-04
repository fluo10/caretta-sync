pub mod server;

pub mod proto {
    use caretta_core::cache::entity::CachedPeerModel;

    tonic::include_proto!("caretta");

    impl From<CachedPeerModel> for CachedPeerMessage {
        fn from(s: CachedPeerModel) -> Self {
            todo!()
        }
    }
}