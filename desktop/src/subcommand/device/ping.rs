use caretta_sync_core::util::RunnableCommand;
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs};
use caretta_sync_core::{
    context::{self, ClientContext},
    proto::api::device::{PingRequest, device_service_client::DeviceServiceClient},
};

use tonic::Request;

#[derive(Debug, Args)]
pub struct DevicePingCommandArgs {
    #[command(flatten)]
    target: DeviceIdentifierArgs,
    #[command(flatten)]
    config: ConfigArgs,
    #[arg(short, long)]
    verbose: bool,
}

impl RunnableCommand for DevicePingCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self
            .config
            .into_parsed_config(app_name)
            .with_default(app_name);
        config.init_tracing_subscriber();
        let context = config.into_client_context(app_name).unwrap();
        let mut client = DeviceServiceClient::connect(&context).await.unwrap();
        let request = Request::new(PingRequest {
            target: Some(self.target.into()),
        });
        let response = client.ping(request).await.unwrap().into_inner();
        println!("{:?}", response)
    }
}
