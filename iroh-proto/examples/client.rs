use clap::Parser;
use iroh_proto::proto::{iroh_client::IrohClient, RemoteInfoIterRequest};
use tokio_stream::StreamExt;
use url::Url;

#[derive(Parser)]
struct Cli {
    tonic_endpoint: Url
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut client = IrohClient::connect(cli.tonic_endpoint.to_string()).await.unwrap();

    println!("Streaming remote info");
    let mut  stream = client.remote_info_iter(RemoteInfoIterRequest{}).await.unwrap().into_inner();
    while let Some(item) = stream.next().await {
        println!("Received: {:?}", item.unwrap())
    }
}