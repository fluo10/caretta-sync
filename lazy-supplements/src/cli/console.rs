use std::time::Duration;

use clap::Args;
use futures::StreamExt;
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use crate::{error::Error, global::GLOBAL};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ConsoleArgs {
    #[command(flatten)]
    config: ConfigArgs,
}

impl ConsoleArgs {
    pub async fn start_console(self) -> Result<(), Error>{
        let _ = crate::global::GLOBAL.get_or_init_node_config(self.config.try_into_node_config().await?).await;
        tokio::spawn( async {
            GLOBAL.launch_swarm().await
        });
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }
}