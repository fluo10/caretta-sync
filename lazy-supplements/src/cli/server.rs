use std::{net::IpAddr, path::PathBuf};

use clap::Args;

use crate::config::PartialServerConfig;

#[derive(Args, Debug)]
pub struct ServerArgs {
    #[command(flatten)]
    server_config: PartialServerConfig,
    #[arg(long)]
    config: PathBuf,
}