use std::{sync::Arc, time::Duration};

use crate::{entity::{authorized_device, invitation_token}, ipc::{DevicePingRequest, DevicePingResponse, IpcContext, IpcActorError, IpcApi, IpcApiTrait, IpcError, IpcMessage}, types::{DeviceIdentifier, InvitationToken}};
use iroh::{Endpoint, discovery::Discovery};
use irpc::{Client, Service, WithChannels};
use n0_future::StreamExt;
use tracing::info;

pub struct IpcActor {
    recv: tokio::sync::mpsc::Receiver<IpcMessage>,
    context: IpcContext
}

impl IpcActor {
    pub fn spawn(context: IpcContext) -> IpcApi {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let actor = Self {
            recv: rx,
            context: context
        };
        n0_future::task::spawn(actor.run());
        IpcApi {
            inner: Client::local(tx),
        }
    }
    async fn run(mut self) {
        while let Some(msg) = self.recv.recv().await {
            self.handle(msg).await;
        }
    }
    async fn handle(&mut self, msg: IpcMessage) {
        match msg {
            IpcMessage::DevicePing(ping) => {
                info!("ping {:?}", ping);
                let WithChannels{tx, inner, ..} = ping;
                let target = inner.target;

                if let Some(x) = tx.send(self.device_ping(target).await.map(|rtt| DevicePingResponse { rtt })).await.err() {
                    tracing::error!("{}", x)
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl IpcApiTrait for IpcActor {
    type Error = IpcActorError;
    async fn device_get(&self, target: DeviceIdentifier) -> Result<authorized_device::Model, Self::Error> {
        todo!();
    }
    
    async fn device_list(&self) -> Result<Vec<authorized_device::Model>, Self::Error>{
        todo!();
    }

    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Self::Error>{
                let public_key = target
            .to_public_key(&self.context)
            .await
            .map_err(|e| IpcActorError::Internal(format!("{:?}", e).to_string()))?
            .ok_or(IpcActorError::DeviceNotFound(target.clone()))?;
        let endpoint: &Endpoint = self.context.as_ref();
        let mut stream = endpoint.discovery().resolve(public_key.into_inner())
            .ok_or(IpcActorError::DeviceNotFound(target.clone()))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(|e| IpcActorError::Internal(format!("{:?}", e).to_string()))?;
            iroh_ping::Ping::new()
                .ping(
                    <IpcContext as AsRef<Endpoint>>::as_ref(&self.context),
                    discovered.into_endpoint_addr(),
                )
                .await
                .map_err(|e| IpcActorError::Internal(format!("{:?}", e)))
        } else {
            unreachable!()
        }
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