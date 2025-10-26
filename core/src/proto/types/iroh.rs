

use crate::proto::{
    ProtoDeserializeError, ProtoSerializeError, types::net::SocketAddr,
};

tonic::include_proto!("caretta_sync.types.iroh");

impl From<iroh::PublicKey> for PublicKey {
    fn from(value: iroh::PublicKey) -> Self {
        Self {
            value: Vec::from(value.as_bytes()),
        }
    }
}

impl TryFrom<PublicKey> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: PublicKey) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.value[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}
