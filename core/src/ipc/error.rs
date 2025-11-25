use caretta_id::CarettaId;
use serde::{Deserialize, Serialize};

use crate::ipc::DeviceIdentifier;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum IpcError {
    #[error("Target device not found: {0}")]
    DeviceNotFound(DeviceIdentifier),
    #[error("Internal error: {0}")]
    Internal(String)
}