use std::time::Duration;

use irpc::{channel::oneshot, rpc_requests};
use serde::{Deserialize, Serialize};

use crate::{ipc::IpcActorError, types::DeviceIdentifier};

type IpcActorResult<T> = Result<T, IpcActorError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicePingRequest {
    pub target: DeviceIdentifier
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicePingResponse {
    pub rtt: Duration
}

#[rpc_requests(message = IpcMessage)]
#[derive(Debug, Serialize, Deserialize)]
pub enum IpcProtocol {
    #[rpc(tx=oneshot::Sender<IpcActorResult<DevicePingResponse>>)]
    DevicePing(DevicePingRequest)
}