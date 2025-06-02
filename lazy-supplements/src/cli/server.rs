use clap::Args;
use futures::StreamExt;
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};
use tracing_subscriber::EnvFilter;

use crate::error::Error;

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServerArgs {
    #[command(flatten)]
    config: ConfigArgs,
}
impl ServerArgs {
    pub async fn start_server(self) -> Result<(), Error>{
        let mut swarm = self.config.try_into_node_config().await?.try_into_swarm().await?;
        loop{
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => println!("{event:?}"),
                _ => {}
            }
        }
    }
}