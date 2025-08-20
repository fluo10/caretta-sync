use std::{net::{IpAddr, Ipv4Addr}, ops, path::{Path, PathBuf}};

use base64::{prelude::BASE64_STANDARD, Engine};
#[cfg(feature="desktop")]
use clap::Args;
use futures::StreamExt;
use libp2p::{identity::{self, DecodingError, Keypair}, noise, ping, swarm::SwarmEvent, tcp, yamux, Swarm};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing_subscriber::EnvFilter;


use crate::{
    config::PartialConfig,
    error::Error, p2p, utils::{emptiable::Emptiable, mergeable::Mergeable}
};

static DEFAULT_P2P_LISTEN_IPS: &[IpAddr] = &[IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))];
static DEFAULT_P2P_PORT: u16 = 0;

fn keypair_to_base64(keypair: &Keypair) -> String {
        let vec = match keypair.to_protobuf_encoding() {
            Ok(x) => x,
            Err(_) => unreachable!(),
        };
        BASE64_STANDARD.encode(vec)
}

fn base64_to_keypair(base64: &str) -> Result<Keypair, Error>  {
        let vec = BASE64_STANDARD.decode(base64)?;
        Ok(Keypair::from_protobuf_encoding(&vec)?)
}

#[derive(Clone, Debug)]
pub struct P2pConfig {
    pub secret: Keypair,
    pub listen_ips: Vec<IpAddr>,
    pub port: u16,
}

impl P2pConfig {
    async fn try_into_swarm (self) -> Result<Swarm<p2p::Behaviour>, Error> {
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(self.secret)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|keypair| p2p::Behaviour::try_from(keypair).unwrap())?
            .build();
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(swarm)
    }
    pub async fn launch_swarm(self) -> Result<(), Error>{
        let mut swarm = self.try_into_swarm().await?;
        loop{
            let swarm_event = swarm.select_next_some().await;
            tokio::spawn(async move{
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => {
                        println!("{event:?}");
                        event.run().await;
                    },
                    _ => {}
                }
            });
        }
    }
}

impl TryFrom<PartialP2pConfig> for P2pConfig {
    type Error = crate::error::Error;
    fn try_from(raw: PartialP2pConfig) -> Result<P2pConfig, Self::Error> {
        Ok(P2pConfig {
            secret: base64_to_keypair(&raw.secret.ok_or(Error::MissingConfig("secret"))?)?,
            listen_ips: raw.listen_ips.ok_or(Error::MissingConfig("listen_ips"))?,
            port: raw.port.ok_or(Error::MissingConfig("port"))?
        })
    }
}

#[cfg_attr(feature="desktop",derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialP2pConfig {
    #[cfg_attr(feature="desktop",arg(long))]
    pub secret: Option<String>,
    #[cfg_attr(feature="desktop",arg(long))]
    pub listen_ips: Option<Vec<IpAddr>>,
    #[cfg_attr(feature="desktop",arg(long))]
    pub port: Option<u16>,
}
impl PartialP2pConfig {
    pub fn with_new_secret(mut self) -> Self {
        self.secret = Some(keypair_to_base64(&Keypair::generate_ed25519()));
        self
    }
}

impl From<P2pConfig> for PartialP2pConfig {
    fn from(config: P2pConfig) -> Self {
        Self {
            secret: Some(keypair_to_base64(&config.secret)),
            listen_ips: Some(config.listen_ips),
            port: Some(config.port)
        }
    }
}

impl Default for PartialP2pConfig {
    fn default() -> Self {
        Self {
            secret: None,
            listen_ips: Some(Vec::from(DEFAULT_P2P_LISTEN_IPS)),
            port: Some(DEFAULT_P2P_PORT),
        }
    }
}

impl Emptiable for PartialP2pConfig {
    fn empty() -> Self {
        Self{
            secret: None,
            listen_ips: None,
            port: None
        }
    }

    fn is_empty(&self) -> bool {
        self.secret.is_none() && self.listen_ips.is_none() && self.port.is_none()
    }
}

impl Mergeable for PartialP2pConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.secret.take() {
            let _ = self.secret.insert(x);
        };
        if let Some(x) = other.listen_ips.take() {
            let _ = self.listen_ips.insert(x);
        };
        if let Some(x) = other.port.take() {
            let _ = self.port.insert(x);
        };
    }
}


#[cfg(test)]
mod tests {
    use libp2p::identity;
    use super::*;
    

    #[tokio::test]
    async fn parse_keypair() {
        let keypair = identity::Keypair::generate_ed25519();
        let keypair2 = base64_to_keypair(&keypair_to_base64(&keypair)).unwrap();

        assert_eq!(keypair.public(), keypair2.public());
    }
    
}
