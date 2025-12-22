use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::{mcp::Api, types::AppInfo, util::RunnableCommand};
use clap::Args;

use crate::args::{ConfigArgs, DeviceIdentifierArgs};

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
    async fn run(self, app_info: AppInfo) {
        let app_name = app_info.app_name;
        let client = self
            .config
            .into_parsed_config(app_name)
            .with_default(app_name)
            .into_client_config(app_name, self.verbose)
            .unwrap()
            .spawn_client(app_info.client_info)
            .await;
        let response = client
            .device_ping(crate::mcp::model::DevicePingRequest {
                target: self.target.into(),
            })
            .await
            .unwrap();
        println!("rtt: {:?}", response.rtt)
    }
}
