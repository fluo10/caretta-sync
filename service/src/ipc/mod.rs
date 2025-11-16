mod device_identifier;

use std::{sync::Arc, time::Duration};

use caretta_sync_core::context::{ServiceContext, ServiceContextExt};
pub use caretta_sync_core::ipc::*;
use irpc::{Client, Service, WithChannels};
use n0_future::StreamExt;
use tracing::info;

use crate::ipc::device_identifier::DeviceIdentifierExt;

struct IpcActor {
    recv: tokio::sync::mpsc::Receiver<IpcMessage>,
    context: Arc<dyn AsRef<ServiceContext> + Sync + Send>
}

impl IpcActor {
    pub fn spawn(context: &Arc<dyn AsRef<ServiceContext> + Sync + Send>) -> IpcApi {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let actor = Self {
            recv: rx,
            context: context.clone()
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
            IpcMessage::PingDevice(ping) => {
                info!("ping {:?}", ping);
                let WithChannels{tx, inner, ..} = ping;
                let target = inner.target;

                if let Some(x) = tx.send(self.ping_device(&target).await).await.err() {
                    tracing::error!("{}", x)
                }
            }
        }
    }

    async fn ping_device(&self, target: &DeviceIdentifier) -> Result<Duration, IpcError> {
        let public_key = target
            .to_public_key()
            .await
            .map_err(|e| IpcError::Internal(format!("{:?}", e).to_string()))?
            .ok_or(IpcError::DeviceNotFound(target.clone()))?;
        let mut stream = self
            .context
            .as_ref()
            .discover(public_key)
            .await
            .ok_or(IpcError::DeviceNotFound(target.clone()))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(|e| IpcError::Internal(format!("{:?}", e).to_string()))?;
            iroh_ping::Ping::new()
                .ping(
                    self.context.as_ref().as_endpoint().unwrap(),
                    discovered.into_endpoint_addr(),
                )
                .await
                .map_err(|e| IpcError::Internal(format!("{:?}", e).to_string()))
        } else {
            unreachable!()
        }
    }
}

