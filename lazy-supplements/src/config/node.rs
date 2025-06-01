use std::{net::IpAddr, path::{Path, PathBuf}};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Args;
use libp2p::identity::{self, DecodingError, Keypair};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};


use crate::{error::Error, global::DEFAULT_DATABASE_FILE_PATH};

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
    secret: Keypair,
    database_path: PathBuf,
    listen_ips: Vec<IpAddr>,
    port: u16,
}

impl Default for NodeConfig {
    fn default() -> NodeConfig{
        NodeConfig {
            secret: identity::Keypair::generate_ed25519(),
            database_path: DEFAULT_DATABASE_FILE_PATH.to_path_buf(),
            listen_ips: DEFAULT_LISTEN_IPS.to_vec(),
            port: DEFAULT_PORT,
        }
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

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct RawNodeConfig {
    #[arg(skip)]
    secret: Option<String>,
    #[arg(long)]
    database_path: Option<PathBuf>,
    #[arg(long)]
    listen_ips: Option<Vec<IpAddr>>,
    #[arg(long)]
    port: Option<u16>,
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
