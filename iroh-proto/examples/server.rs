#[cfg(unix)]
use std::net::SocketAddr;
use std::{net::{IpAddr, Ipv4Addr}, path::PathBuf, sync::Arc};

use clap::Parser;
use tokio::time::error;
use tonic::transport::Server;
use url::{form_urlencoded::Parse, Host, Url};

#[derive(Parser)]
struct Cli {
    tonic_endpoint: Url
}

enum ParsedUrl {
    #[cfg(unix)]
    Tcp(SocketAddr),
    Unix(PathBuf)
}

impl From<Url> for ParsedUrl {
    fn from(url: Url) -> Self {
        match url.scheme() {
            #[cfg(unix)]
            "unix" | "file" => {
                ParsedUrl::Unix(url.to_file_path().expect("Invalid path url"))
            },
            "http" | "tcp" => {
                ParsedUrl::Tcp(url.socket_addrs(|| None).expect("Invalid address and port").pop().expect("Target domain is not found"))
            },
            _ => panic!("Invalid url scheme")
        }
    } 
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let endpoint = iroh::Endpoint::builder().discovery_n0().discovery_dht().discovery_local_network().bind().await.unwrap();
    let server = Server::builder()
        .add_service(iroh_proto::proto::iroh_server::IrohServer::new(iroh_proto::server::IrohServer::from(endpoint)));
    let _ = match ParsedUrl::from(args.tonic_endpoint) {
        #[cfg(unix)]
        ParsedUrl::Unix(x) => {
            use tokio::net::UnixListener;
            use tokio_stream::wrappers::UnixListenerStream;
            if x.exists() {
                if x.is_file() {
                    println!("Socket file already exists. Removing.");
                    std::fs::remove_file(&x).expect("Failed to remove target file already exists");
                } else if x.is_dir() {
                    panic!("Directory already exists");
                }
            }
            let uds = UnixListener::bind(x).expect("Failed to bind the path");
            let uds_stream = UnixListenerStream::new(uds);
            server.serve_with_incoming(uds_stream).await
        },
        ParsedUrl::Tcp(x) => {
            server.serve(x).await
        }
    };
}