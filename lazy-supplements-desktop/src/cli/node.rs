use std::{net::IpAddr, ops::Mul, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};
use libp2p::{
    multiaddr::Protocol, noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId
};

use crate::{cli::ServerArgs, error::{CoreError, DesktopError, Error}};

use super::ConfigArgs;

#[derive(Debug, Args)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub command: NodeCommand
}

#[derive(Debug, Parser)]
pub struct ConsoleNodeArgs {
    #[command(flatten)]
    pub args: NodeArgs,
}

impl NodeArgs {
    pub async fn run(self) -> Result<(), Error> {
        println!("{self:?}");
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct PeerArgs {
    #[arg(value_parser = clap::value_parser!(PeerArg))]
    pub peer: PeerArg,
}
#[derive(Clone, Debug)]
pub enum PeerArg {
    Addr(Multiaddr),
    Id(PeerId),
    Number(u32),
}

impl FromStr for PeerArg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<Multiaddr>() {
            Ok(Self::Addr(x))
        } else if let Ok(x) = s.parse::<PeerId>() {
            Ok(Self::Id(x))
        } else if let Ok(x) = s.parse::<u32>() {
            Ok(Self::Number(x))
        } else {
            Err(format!("Invalid value: {s}").to_string())
        }
    }
}


#[derive(Args, Debug)]
pub struct NodeJoinArgs {
    #[command(flatten)]
    pub peer: PeerArgs,
    pub pass: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommand {
    Add(PeerArgs),
    Ping(PeerArgs),
    Join(PeerArgs),
    List,
    Delete(PeerArgs),
}


impl PeerArgs {
    pub async fn run(self) -> Result<(), Error> {
        println!("{self:?}");
        todo!()
    }
}