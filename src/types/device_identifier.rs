use caretta_id::CarettaId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "engine")]
use crate::{ipc::IpcActorError, types::EndpointPublicKey};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceIdentifier {
    Id(CarettaId),
    Name(String),
    PublicKey(EndpointPublicKey)
}

#[cfg(feature = "engine")]
impl DeviceIdentifier {
    pub async fn to_public_key<C>(&self, ctx: C) -> Result<Option<EndpointPublicKey>, IpcActorError> {
        match self {
            DeviceIdentifier::Id(x) => todo!(),
            DeviceIdentifier::Name(x) => todo!(),
            DeviceIdentifier::PublicKey(x) => Ok(Some(x.clone())),
        }

    }
}

impl std::fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceIdentifier::Id(x) => write!(f, "id: {}", x),
            DeviceIdentifier::Name(x) => write!(f, "name: {}", x),
            DeviceIdentifier::PublicKey(x) => write!(f,"public_key: {}", x)
        }
    }
}

