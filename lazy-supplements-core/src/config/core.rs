use std::{net::IpAddr, ops, path::{Path, PathBuf}};

use base64::{prelude::BASE64_STANDARD, Engine};
#[cfg(feature="desktop")]
use clap::Args;
use libp2p::{identity::{self, DecodingError, Keypair}, noise, ping, tcp, yamux, Swarm};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing_subscriber::EnvFilter;


use crate::{
    config::PartialConfig,
    error::Error, p2p
};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoreConfig {
    #[serde(with = "keypair_parser")]
    pub secret: Keypair,
    pub listen_ips: Vec<IpAddr>,
    pub port: u16,
}

impl CoreConfig {
    pub async fn try_into_swarm (self) -> Result<Swarm<p2p::Behaviour>, Error> {
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
}

impl TryFrom<PartialCoreConfig> for CoreConfig {
    type Error = Error;
    fn try_from(raw: PartialCoreConfig) -> Result<CoreConfig, Self::Error> {
        Ok(CoreConfig {
            secret: base64_to_keypair(&raw.secret.ok_or(Error::MissingConfig("secret"))?)?,
            listen_ips: raw.listen_ips.ok_or(Error::MissingConfig("listen_ips"))?,
            port: raw.port.ok_or(Error::MissingConfig("port"))?
        })
    }
}

mod keypair_parser {
    use libp2p::identity::Keypair;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(keypair: &Keypair, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        serializer.serialize_str(&super::keypair_to_base64(keypair))
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Keypair, D::Error>
    where D: Deserializer<'de>
    {
        match super::base64_to_keypair(&String::deserialize(deserializer)?) {
            Ok(x) => Ok(x),
            Err(crate::error::Error::Base64Decode(_)) => Err(serde::de::Error::custom("Decoding base64 error")),
            Err(_) => unreachable!()
        }
    }
}

#[cfg_attr(feature="desktop",derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialCoreConfig {
    #[cfg_attr(feature="desktop",arg(long))]
    pub secret: Option<String>,
    #[cfg_attr(feature="desktop",arg(long))]
    pub listen_ips: Option<Vec<IpAddr>>,
    #[cfg_attr(feature="desktop",arg(long))]
    pub port: Option<u16>,
}
impl PartialCoreConfig {

    pub fn with_new_secret(mut self) -> Self {
        self.secret = Some(keypair_to_base64(&Keypair::generate_ed25519()));
        self
    }
    pub async fn read_or_create<T>(path: T) -> Result<Self, Error> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::empty().write_to(&path).await?;
        }
        Self::read_from(&path).await
    }
    pub async fn read_from<T>(path:T) -> Result<Self, Error> 
    where 
    T: AsRef<Path>
    {
        let mut file = File::open(path.as_ref()).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    pub async fn write_to<T>(&self, path:T) -> Result<(), Error> 
    where 
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path).await?;
        }
        let mut file = File::create(&path).await?;
        file.write_all(toml::to_string(self)?.as_bytes()).await?;
        Ok(())
    }
}

impl From<CoreConfig> for PartialCoreConfig {
    fn from(config: CoreConfig) -> Self {
        Self {
            secret: Some(keypair_to_base64(&config.secret)),
            listen_ips: Some(config.listen_ips),
            port: Some(config.port)
        }
    }
}
impl PartialConfig<CoreConfig> for PartialCoreConfig {
    fn empty() -> Self {
        Self {
            secret: None,
            listen_ips: None,
            port: None,
        }
    }
    fn merge(&mut self, another: Self) {
        if let Some(x) = another.secret {
            self.secret = Some(x);
        };
        if let Some(x) = another.listen_ips {
            self.listen_ips = Some(x);
        };
        if let Some(x) = another.port {
            self.port = Some(x);
        };
    }
    
    fn default() -> Self {
        todo!()
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
