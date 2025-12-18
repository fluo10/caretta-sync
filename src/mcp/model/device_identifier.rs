use caretta_id::CarettaId;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::mcp::McpError;
use crate::types::EndpointPublicKey;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum DeviceIdentifier {
    Id(CarettaId),
    Name(String),
    PublicKey(EndpointPublicKey)
}

#[cfg(feature = "server")]
impl DeviceIdentifier {
    pub async fn to_public_key<C>(&self, ctx: C) -> Result<Option<EndpointPublicKey>, McpError> {
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

