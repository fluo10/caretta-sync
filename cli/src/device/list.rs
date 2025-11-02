use crate::option::ConfigOptionArgs;
use caretta_sync_core::{proto::api::device::{device_service_client::DeviceServiceClient, list_request::Status, ListRequest}, utils::runnable::Runnable};
use clap::Args;
use iroh::EndpointId;
use mtid::Dtid;
use tonic::Request;


#[derive(Debug, Args)]
#[group(multiple = false)]
struct FilterOptionArgs {
    #[arg(long)]
    discovered: bool,
    #[arg(long)]
    all: bool,
}

impl From<FilterOptionArgs> for ListRequest {
    fn from(value: FilterOptionArgs) -> Self {
        let statuses: Vec<i32> = match (value.discovered, value.all) {
            (true, true) => unreachable!(),
            (true, false) => vec![Status::Discovered.into()],
            (false, true) => vec![Status::Discovered.into(), Status::Authorized.into()],
            (false, false) => vec![Status::Authorized.into()]
        };
        ListRequest { statuses }
    }
}

#[derive(Debug, Args)]
pub struct DeviceListCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(flatten)]
    filter: FilterOptionArgs,
}

impl Runnable for DeviceListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let mut client = DeviceServiceClient::connect(&self.config.into_client_context(app_name)).await.unwrap();

        let list_request = ListRequest::from(self.filter);
        let request = Request::new(list_request);
        let response = client.list(request).await.unwrap().into_inner();
        println!("{:?}", response.items)
    }
}
