use caretta_id::CarettaId;
use serde::{Deserialize, Serialize};

use crate::types::DeviceIdentifier;

#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error(transparent)]
    Actor(#[from] IpcActorError),
    #[error("RpcError: {0}")]
    Rpc(#[from] irpc::Error),
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum IpcActorError {
    #[error("Target device not found: {0}")]
    DeviceNotFound(DeviceIdentifier),
    #[error("Internal error: {0}")]
    Internal(String)
}