use std::collections::HashSet;

use libp2p::bytes::buf::UninitSlice;
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard};

use crate::cache::entity::PeerModel;

static UNINITIALIZED_MESSAGE: &str = "Global peer set uninitialized!";
pub trait GlobalPeers {
    fn get_peers_once_cell(&'static self) -> &OnceCell<RwLock<HashSet<PeerModel>>>;
    async fn write_peers(&'static self) -> tokio::sync::RwLockWriteGuard<'_ ,HashSet<PeerModel>> {
        self.get_peers_once_cell().get().expect(UNINITIALIZED_MESSAGE).write().await
    }
    async fn read_peers(&'static self) -> RwLockReadGuard<'_, HashSet<PeerModel>> {
        self.get_peers_once_cell().get().expect(UNINITIALIZED_MESSAGE).read().await

    }
}