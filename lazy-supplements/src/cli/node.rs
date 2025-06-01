use std::{net::IpAddr, path::PathBuf};

use clap::{Args, Subcommand};
use futures::StreamExt;
use libp2p::{
    multiaddr::Protocol, noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr
};
use tracing_subscriber::EnvFilter;

use crate::error::Error;

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    command: NodeCommand
}

#[derive(Args, Debug)]
pub struct JoinNodeArgs {
    #[arg(long)]
    endpoint: IpAddr,
    #[arg(long)]
    port: u16,
    #[arg(long)]
    peer_id: String,
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommand {
    Ping(JoinNodeArgs),
    Join(JoinNodeArgs),
}


impl JoinNodeArgs {
    pub async fn ping(self) -> Result<(), Error> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| ping::Behaviour::default())?
            .build();
            
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        

        let mut remote: Multiaddr = Multiaddr::empty();
        remote.push(match self.endpoint {
            IpAddr::V4(x) => Protocol::Ip4(x),
            IpAddr::V6(x) => Protocol::Ip6(x),
        });
        remote.push(Protocol::Tcp(self.port));
        let addr = remote.to_string();
        swarm.dial(remote)?;
        println!("Dialed {addr}");

        loop{
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => println!("{event:?}"),
                _ => {}
            }
        }
    }
}