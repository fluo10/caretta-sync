use std::path::PathBuf;

use clap::Args;
use libp2p::identity;

use crate::{config::RawNodeConfig, global::DEFAULT_CONFIG_FILE_PATH};

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(short, long)]
    force: bool,
    #[command(flatten)]
    node_config: RawNodeConfig,
}

impl InitArgs {
    pub async fn init_config(self) {
        let config_path = if let Some(x) = self.config {
            x
        } else {
            DEFAULT_CONFIG_FILE_PATH.to_path_buf()
        };
        if config_path.exists() && !self.force {
            println!("Config file already exists!");
            return;
        } else {
            let config = self.node_config.with_new_secret();
            config.write_to(config_path).await.unwrap()
        }

    }
}