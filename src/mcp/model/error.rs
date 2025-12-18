#[cfg(feature = "server")]
use iroh::discovery::DiscoveryError;
use rmcp::ErrorData;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

use crate::mcp::model::DeviceIdentifier;

/// Error returned from McpServer.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum McpError {
    #[error("Target device not found: {0}")]
    DeviceNotFound(DeviceIdentifier),
    #[error("Device discovery failed: {0}")]
    DeviceDiscoveryFailed(String),
    #[error("iroh-ping failed: {0}")]
    DevicePingFailed(String)
}

impl From<McpError> for ErrorData {
    
    fn from(value: McpError) -> Self {
        let data = match serde_json::to_value(&value) {
            Ok(x) => x,
            Err(_) => serde_json::Value::String(format!("{:?}", &value))
        };
        let msg = format!("{:?}", &value);
        ErrorData::internal_error(msg, Some(data))
    }
}

#[cfg(feature = "server")]
impl From<DiscoveryError> for McpError {
    fn from(value: DiscoveryError) -> Self {
        match value {
            DiscoveryError::NoResults { endpoint_id, meta } => McpError::DeviceNotFound(DeviceIdentifier::PublicKey(endpoint_id.into())),
            x => McpError::DeviceDiscoveryFailed(format!("{:?}", x)),
        }
    }
}