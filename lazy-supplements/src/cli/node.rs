use std::{net::IpAddr, path::PathBuf};

use clap::{Args, Subcommand};
use futures::StreamExt;
use libp2p::{
    multiaddr::Protocol, noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr
};
use tracing_subscriber::EnvFilter;

use crate::{cli::ServerArgs, error::Error};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub command: NodeCommand
}

#[derive(Args, Debug)]
pub struct JoinNodeArgs {
    #[arg(long)]
    pub peer_ip: IpAddr,
    #[arg(long)]
    pub peer_port: u16,
    //#[arg(long)]
    //pub peer_id: String,
    #[command(flatten)]
    pub config: ConfigArgs,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommand {
    Ping(JoinNodeArgs),
    Join(JoinNodeArgs),
}


impl JoinNodeArgs {
    pub async fn ping(self) -> Result<(), Error> {
        let mut swarm = self.config.try_into_node_config().await?.try_into_swarm().await?;

        let mut remote: Multiaddr = Multiaddr::empty();
        remote.push(match self.peer_ip {
            IpAddr::V4(x) => Protocol::Ip4(x),
            IpAddr::V6(x) => Protocol::Ip6(x),
        });
        remote.push(Protocol::Tcp(self.peer_port));
        let addr = remote.to_string();
        swarm.dial(remote)?;
        println!("Dialed {addr}");

        loop{
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => {
                    println!("{event:?}");
                    event.run().await;
                },
                _ => {}
            }
        }
    }
}