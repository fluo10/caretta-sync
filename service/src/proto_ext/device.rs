use caretta_sync_core::proto::{ProtoDeserializeError, api::device::*};
use iroh::PublicKey;

#[async_trait::async_trait]
impl DeviceIdentifierExt for Identifier {
    async fn to_public_key(&self) -> Result<Option<PublicKey>, ProtoDeserializeError> {
        use identifier::Value;
        if let Some(x) = self.value.as_ref() {
            match x {
                Value::Id(y) => todo!(),
                Value::Name(y) => todo!(),
                Value::PublicKey(y) => Ok(Some((y).try_into()?)),
            }
        } else {
            Err(ProtoDeserializeError::MissingField("value"))
        }
    }
}

/// An extension trait for [`Identifier`]
#[async_trait::async_trait]
pub trait DeviceIdentifierExt {
    async fn to_public_key(&self) -> Result<Option<PublicKey>, ProtoDeserializeError>;
}
