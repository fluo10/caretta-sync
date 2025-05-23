use std::{net::IpAddr, path::PathBuf};

use clap::Args;

#[derive(Args, Debug)]
pub struct ServerArgs {
    #[arg(long)]
    listen_ip: IpAddr,
    #[arg(long)]
    port: i32,
    #[arg(long)]
    config: PathBuf,
}