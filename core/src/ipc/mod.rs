mod error;
use std::{fmt::Display, net::SocketAddr, time::Duration};

use caretta_id::CarettaId;
use iroh::PublicKey;
use irpc::{Client, channel::oneshot, rpc::RemoteService, rpc_requests};
use n0_future::task::{self, AbortOnDropHandle};
use quinn::Endpoint;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
pub use error::IpcError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceIdentifier {
    Id(CarettaId),
    Name(String),
    PublicKey(PublicKey)
}

impl Display for DeviceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceIdentifier::Id(x) => write!(f, "id: {}", x),
            DeviceIdentifier::Name(x) => write!(f, "name: {}", x),
            DeviceIdentifier::PublicKey(x) => write!(f,"public_key: {}", x)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingDevice {
    pub target: DeviceIdentifier
}

#[rpc_requests(message = IpcMessage)]
#[derive(Debug, Serialize, Deserialize)]
pub enum IpcProtocol {
    #[rpc(tx=oneshot::Sender<Result<Duration, IpcError>>)]
    PingDevice(PingDevice)
}

pub struct IpcApi {
    pub inner: Client<IpcProtocol>,
}

impl IpcApi {
    pub fn connect (endpoint: Endpoint, addr: SocketAddr) -> Result<Self, CoreError> {
        Ok(IpcApi {
            inner: Client::quinn(endpoint, addr)
        })
    } 
    pub fn listen(&self, endpoint: Endpoint) -> Result<AbortOnDropHandle<()>, CoreError> {
        let local = self
            .inner
            .as_local()
            .expect("cannot listen on remote API");
        let join_handle = task::spawn(irpc::rpc::listen(
            endpoint,
            IpcProtocol::remote_handler(local),
        ));
        Ok(AbortOnDropHandle::new(join_handle))
    }
    pub async fn ping_device(&self, target: DeviceIdentifier) -> irpc::Result<Result<Duration, IpcError>>  {
        self.inner.rpc(PingDevice{target}).await
    }
}