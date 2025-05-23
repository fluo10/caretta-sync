use std::{net::IpAddr, path::PathBuf};

use clap::Args;

#[derive(Args, Debug)]
pub struct ConnectArgs {
    #[arg(long)]
    endpoint: IpAddr,
    #[arg(long)]
    port: i32,
    #[arg(long)]
    config: PathBuf,
}