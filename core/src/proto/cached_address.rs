use libp2p::Multiaddr;

use crate::cache::entity::CachedAddressModel;
use crate::utils::utc_to_timestamp;
use crate::proto::CachedAddressMessage;

impl From<&CachedAddressModel> for CachedAddressMessage {
    fn from(a: &CachedAddressModel) -> Self {
        Self {
            number: a.id,
            created_at: Some(utc_to_timestamp(&a.created_at)),
            updated_at: Some(utc_to_timestamp(&a.updated_at)),
            multiaddress: Multiaddr::from(a.multiaddress.clone()).to_string(),
        }
    }
}