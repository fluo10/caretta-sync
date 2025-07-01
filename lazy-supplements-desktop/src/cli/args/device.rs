use clap::Args;
use libp2p::{Multiaddr, PeerId};
use uuid::Uuid;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceArgs {
    device_number: Option<u32>,
    device_id: Option<Uuid>,
    peer_id: Option<PeerId>,
    multiaddr: Option<Multiaddr>,
}