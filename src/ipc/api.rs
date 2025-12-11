use std::{net::SocketAddr, time::Duration};

use irpc::{Client, rpc::RemoteService};
use n0_future::task::{self, AbortOnDropHandle};
use quinn::Endpoint;

use crate::{entity::{authorized_device, invitation_token}, ipc::{DevicePingRequest, DevicePingResponse, IpcApiTrait, IpcError, IpcProtocol, IpcResult}, types::{DeviceIdentifier, InvitationToken}};

type IpcApiResult<T> = Result<T, IpcError>;
pub struct IpcApi {
    pub inner: Client<IpcProtocol>,
}

impl IpcApi {
    pub fn connect (endpoint: Endpoint, addr: SocketAddr) -> Result<Self, IpcError> {
        Ok(IpcApi {
            inner: Client::quinn(endpoint, addr)
        })
    } 
    pub fn listen(&self, endpoint: Endpoint) -> Result<AbortOnDropHandle<()>, IpcError> {
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

}

#[async_trait::async_trait]
impl IpcApiTrait for IpcApi {
    type Error = IpcError;

    async fn device_get(&self, target: DeviceIdentifier) -> Result<authorized_device::Model, Self::Error> {
        todo!();
    }
    
    async fn device_list(&self) -> Result<Vec<authorized_device::Model>, Self::Error>{
        todo!();
    }

    async fn device_ping(&self, target: DeviceIdentifier) -> IpcApiResult<Duration>  {
        Ok(self.inner.rpc(DevicePingRequest { target }).await??.rtt)
    }

    async fn device_remove(&self, target: DeviceIdentifier) -> Result<(), Self::Error> {
        todo!();
    }

    async fn token_get(&self, id: u32) -> Result<invitation_token::Model, Self::Error>{
        todo!();
    }

    async fn token_list(&self) -> Result<Vec<invitation_token::Model>, Self::Error> {
        todo!();
    }

    async fn token_revoke(&self, id: u32) -> Result<(), Self::Error> {
        todo!();
    }

    async fn init(&self) -> Result<(), Self::Error> {
        todo!();
    }
    
    async fn invite(&self) -> Result<InvitationToken, Self::Error> {
        todo!();
    }

    async fn join(&self, token: InvitationToken) -> Result<(), Self::Error> {
        todo!();
    }
}