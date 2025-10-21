use clap::Args;
use iroh::PublicKey;
use libp2p::{Multiaddr, PeerId};
use mtid::Dtid;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct PeerIdentifierArgs {
    peer_id: Option<Dtid>,
    public_key: Option<PublicKey>,
}