use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use caretta_sync_core::{ipc::IpcApi, util::RunnableCommand};
use clap::Args;
use irpc::util::{make_client_endpoint, make_insecure_client_endpoint};

use crate::args::{ConfigArgs, DeviceIdentifierArgs};
use caretta_sync_core::{
    context::{self, ClientContext},
};


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
        let api = match context.ipc_config.endpoint.clone() {
            SocketAddr::V4(x) => {
                let client = make_insecure_client_endpoint(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0).into()).unwrap();
                IpcApi::connect(client, x.into()).unwrap()
            },
            SocketAddr::V6(x) => {
                let client = make_insecure_client_endpoint(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0).into()).unwrap();
                IpcApi::connect(client, x.into()).unwrap()
            }
        };

        let rtt = api.ping_device(self.target.into()).await.unwrap().unwrap();
        println!("rtt: {:?}", rtt)
    }
}
