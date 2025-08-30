use clap::Args;
use caretta_sync_core::{
    utils::runnable::Runnable,
    proto::*,
};
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct PeerListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs
}

impl Runnable for PeerListCommandArgs {
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self.config.into_config(app_name).await;
        let url = config.rpc.endpoint_url.to_string();
        let mut client = caretta_sync_core::proto::cached_peer_service_client::CachedPeerServiceClient::connect(url).await.expect("Target endpoint should be accessible");
        let request = tonic::Request::new(CachedPeerListRequest {});
        let response = client.list(request).await.expect("Faild to request/response");
        println!("{:?}", response);
    }
}