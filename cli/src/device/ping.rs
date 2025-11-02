use crate::option::{ConfigOptionArgs, DeviceIdentifierArgs};
use caretta_sync_core::{proto::api::device::{PingRequest, device_service_client::DeviceServiceClient}, utils::runnable::Runnable};
use clap::Args;
use tonic::Request;

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs {
    #[command(flatten)]
    target: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl Runnable for DevicePingCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut client = DeviceServiceClient::connect(&self.config.into_client_context(app_name)).await.unwrap();
        let request = Request::new(PingRequest{
            target: Some(self.target.into())
        });
        let response = client.ping(request).await.unwrap().into_inner();
        println!("{:?}", response)
    }
}
