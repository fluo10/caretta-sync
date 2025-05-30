use std::{net::IpAddr, path::PathBuf};

use clap::{Args, Subcommand};
use libp2p::PeerId;

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
    port: i32,
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


