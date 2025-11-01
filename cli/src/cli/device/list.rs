use crate::cli::ConfigOptionArgs;
use caretta_sync_core::{proto::api::device::{device_service_client::DeviceServiceClient, list_request::Status, ListRequest}, utils::runnable::Runnable};
use clap::Args;
use iroh::EndpointId;
use mtid::Dtid;
use tonic::Request;

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl Runnable for DeviceListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let client = DeviceServiceClient::connect(&self.config.into_client_context(app_name)).await.unwrap();
        let request = Request::new(ListRequest{ statuses: vec![Status::Verified.into()] });
        let response = client.list(request).await.unwrap().into_inner();
        println!("device_id, device_name, endpoint_id, status");
        for item in response.items {
            println!("{}, {}, {}, {}", Dtid::from_proto_lossy(item.device_id.unwrap()), item.device_name, EndpointId::try_from(item.endpoint_id.unwrap()).unwrap(), item.status.unwrap())
        }
    }
}
