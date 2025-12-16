use serde::{Deserialize, Serialize};

use crate::mcp::model::DeviceIdentifier;

#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error(transparent)]
    Actor(#[from] IpcEngineError),
    #[error("RpcError: {0}")]
    Mpc(#[from] rmcp::ErrorData),
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum IpcEngineError {
    #[error("Target device not found: {0}")]
    DeviceNotFound(DeviceIdentifier),
    #[error("Internal error: {0}")]
    Internal(String)
}