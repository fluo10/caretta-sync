use std::fmt::Display;

use crate::proto::ProtoDeserializeError;

tonic::include_proto!("caretta_sync.types.iroh");

impl From<iroh::PublicKey> for PublicKey {
    fn from(value: iroh::PublicKey) -> Self {
        Self {
            value: Vec::from(value.as_bytes()),
        }
    }
}
impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        iroh::PublicKey::try_from(self).map_err(|_| std::fmt::Error)?.fmt(f)
    }
}
impl TryFrom<&PublicKey> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.value[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}