use crate::{RunnableCommand, option::{ConfigOptionArgs, DeviceIdentifierArgs}};
use caretta_sync_core::{context, proto::api::device::{PingRequest, device_service_client::DeviceServiceClient}};

use clap::Args;
use tonic::Request;

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs {
    #[command(flatten)]
    target: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[arg(short, long)]
    verbose: bool
}

impl RunnableCommand for DevicePingCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let context = self.config.into_client_context(app_name);
        context.init_tracing_subscriber(self.verbose);
        let mut client = DeviceServiceClient::connect(&context).await.unwrap();
        let request = Request::new(PingRequest{
            target: Some(self.target.into())
        });
        let response = client.ping(request).await.unwrap().into_inner();
        println!("{:?}", response)
    }
}
