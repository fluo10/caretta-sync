use crate::{cache::entity::{CachedAddressModel, CachedPeerModel}, proto::{CachedAddressMessage, CachedPeerMessage}, utils::utc_to_timestamp};

impl From<(&CachedPeerModel, &Vec<CachedAddressModel>)> for CachedPeerMessage {
    fn from(source: (&CachedPeerModel, &Vec<CachedAddressModel>)) -> Self {
        let (peer, addresses) = source;
        
        Self {
            number: peer.id,
            peer_id: peer.peer_id.to_string(),
            created_at: Some(utc_to_timestamp(&peer.created_at)),
            addresses: addresses.iter().map(|x| CachedAddressMessage::from(x)).collect(),
        }
    }
}