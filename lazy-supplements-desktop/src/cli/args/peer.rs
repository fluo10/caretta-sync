use clap::Args;
use libp2p::{Multiaddr, PeerId};

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct PeerArgs {
    cache_number: Option<u32>,
    peer_id: Option<PeerId>,
    multiaddr: Option<Multiaddr>,
}