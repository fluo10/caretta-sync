use caretta_sync_core::ipc::DeviceIdentifier;
use iroh::PublicKey;
use irpc::Service;

use crate::error::ServiceError;

#[async_trait::async_trait]
pub trait DeviceIdentifierExt {
    async fn to_public_key(&self) -> Result<Option<PublicKey>, ServiceError>;
}

#[async_trait::async_trait]
impl DeviceIdentifierExt for DeviceIdentifier {
    async fn to_public_key(&self) -> Result<Option<PublicKey>, ServiceError> {
        match self {
            DeviceIdentifier::Id(x) => todo!(),
            DeviceIdentifier::Name(x) => todo!(),
            DeviceIdentifier::PublicKey(x) => Ok(Some(x.clone()))
        }
    }
}