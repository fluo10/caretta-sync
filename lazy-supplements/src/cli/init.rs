use std::path::PathBuf;

use clap::Args;
use libp2p::identity;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(long)]
    config: Option<PathBuf>
}

impl InitArgs {
    fn main(self) {
        let config_path = if let Some(x) = self.config {
            x
        } else {
            crate::cli::default_config_path()
        };
        if config_path.exists() {
            println!("Config file already exists!");
            return;
        } else {
            let keypair = identity::Keypair::generate_ed25519();
            let buf = keypair.to_protobuf_encoding().unwrap();
            
        }

    }
}