use std::collections::HashSet;

use libp2p::bytes::buf::UninitSlice;
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard};

use crate::cache::entity::PeerModel;

use super::GlobalRwLock;

pub static PEERS: GlobalRwLock<HashSet<PeerModel>> = GlobalRwLock::const_new(stringify!(PEERS));

