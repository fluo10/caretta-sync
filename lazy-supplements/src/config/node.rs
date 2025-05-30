use std::path::PathBuf;

use libp2p::identity::{self, Keypair};
use serde::{Deserialize, Serialize};

use crate::global::DEFAULT_DATABASE_FILE_PATH;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    #[serde(with = "keypair")]
    secret: Keypair,
    database_path: Option<PathBuf>
}

impl NodeConfig {
    pub fn new() -> Self {
        Self {
            secret: identity::Keypair::generate_ed25519(),
            database_path: None,
        }
    }
    pub fn get_database_path(&self) -> PathBuf {
        if let Some(x) = self.database_path.clone() {
            x
        } else {
            DEFAULT_DATABASE_FILE_PATH.clone()
        }
    }
}

mod keypair {
    use base64::{prelude::BASE64_STANDARD, Engine};
    use libp2p::identity::Keypair;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(keypair: &Keypair, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let vec = keypair.to_protobuf_encoding().unwrap();
        let base64 = BASE64_STANDARD.encode(vec);
        serializer.serialize_str(&base64)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Keypair, D::Error>
    where D: Deserializer<'de>
    {
        let base64 = String::deserialize(deserializer)?;
        let vec = BASE64_STANDARD.decode(base64).unwrap();
        Ok(Keypair::from_protobuf_encoding(&vec).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use libp2p::identity;
    use super::*;
    

    #[tokio::test]
    async fn serialize_deserialize() {
        let keypair = identity::Keypair::generate_ed25519();
        let config = NodeConfig {
            secret: keypair.clone(),
            database_path: None,
        };
        let string = toml::to_string(&config).unwrap();
        println!("Parsed config: {}", &string);
        let parsed_config: NodeConfig = toml::from_str(&string).unwrap();
        assert_eq!(keypair.public(), parsed_config.secret.public());
    }
}
