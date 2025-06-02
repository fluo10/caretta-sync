use std::{net::IpAddr, ops, path::{Path, PathBuf}};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Args;
use libp2p::{identity::{self, DecodingError, Keypair}, noise, ping, tcp, yamux, Swarm};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing_subscriber::EnvFilter;


use crate::{cli::ConfigArgs, error::Error, global::DEFAULT_DATABASE_FILE_PATH};

use super::{PartialConfig, DEFAULT_LISTEN_IPS, DEFAULT_PORT};

fn keypair_to_base64(keypair: &Keypair) -> Result<String, Error> {
        let vec = keypair.to_protobuf_encoding()?;
        let base64 = BASE64_STANDARD.encode(vec);
        Ok(base64)
}

fn base64_to_keypair(base64: &str) -> Result<Keypair, Error>  {
        let vec = BASE64_STANDARD.decode(base64)?;
        Ok(Keypair::from_protobuf_encoding(&vec)?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    #[serde(with = "keypair_parser")]
    pub secret: Keypair,
    pub database_path: PathBuf,
    pub listen_ips: Vec<IpAddr>,
    pub port: u16,
}

impl NodeConfig {
    pub async fn try_into_swarm (self) -> Result<Swarm<ping::Behaviour>, Error> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(self.secret)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| ping::Behaviour::default())?
            .build();
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(swarm)
    }
}

impl TryFrom<RawNodeConfig> for NodeConfig {
    type Error = Error;
    fn try_from(raw: RawNodeConfig) -> Result<NodeConfig, Self::Error> {
        Ok(NodeConfig {
            secret: base64_to_keypair(&raw.secret.ok_or(Error::MissingConfig("secret"))?)?,
            database_path: raw.database_path.ok_or(Error::MissingConfig("database_path"))?,
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
        match super::keypair_to_base64(keypair) {
            Ok(x) => serializer.serialize_str(&x),
            Err(_) => Err(serde::ser::Error::custom("Decoding keypair error"))
        }
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

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct RawNodeConfig {
    #[arg(skip)]
    pub secret: Option<String>,
    #[arg(long)]
    pub database_path: Option<PathBuf>,
    #[arg(long)]
    pub listen_ips: Option<Vec<IpAddr>>,
    #[arg(long)]
    pub port: Option<u16>,
}
impl RawNodeConfig {

    pub fn with_new_secret(mut self) -> Self {
        self.secret = Some(keypair_to_base64(&Keypair::generate_ed25519()).unwrap());
        self
    }

    pub fn new() -> Self {
        RawNodeConfig {
            secret: None,
            database_path: None,
            listen_ips: None,
            port: None,
        }
    }

    pub async fn read_or_create<T>(path: T) -> Result<Self, Error> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path).await?;
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
        let config: RawNodeConfig = toml::from_str(&content)?;
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

    pub fn merge(&mut self, another: RawNodeConfig) {
        if let Some(x) = another.secret {
            self.secret = Some(x);
        };
        if let Some(x) = another.database_path {
            self.database_path = Some(x);
        };
        if let Some(x) = another.listen_ips {
            self.listen_ips = Some(x);
        };
        if let Some(x) = another.port {
            self.port = Some(x);
        };
    }
}

impl ops::Add<RawNodeConfig> for RawNodeConfig {
    type Output = RawNodeConfig;
    fn add(mut self, another: RawNodeConfig) -> RawNodeConfig {
        self.merge(another);
        self
    }
}



#[cfg(test)]
mod tests {
    use libp2p::identity;
    use super::*;
    

    #[tokio::test]
    async fn parse_keypair() {
        let keypair = identity::Keypair::generate_ed25519();
        let keypair2 = base64_to_keypair(&keypair_to_base64(&keypair).unwrap()).unwrap();

        assert_eq!(keypair.public(), keypair2.public());
    }
}
