use automerge::ActorId;
use libp2p::identity::{self, Keypair};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    #[serde(with = "keypair")]
    secret: Keypair,
    #[serde(with = "actor_id")]
    actor_id: ActorId
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

mod actor_id {
    use automerge::ActorId;
    use base64::{prelude::BASE64_STANDARD, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(actor_id: &ActorId, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let bytes = actor_id.to_bytes();
        let base64 = BASE64_STANDARD.encode(bytes);
        serializer.serialize_str(&base64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ActorId, D::Error>
    where D: Deserializer<'de>
    {
        let base64 = String::deserialize(deserializer)?;
        let vec = BASE64_STANDARD.decode(base64).unwrap();
        Ok(ActorId::from(vec))
    }
}

#[cfg(test)]
mod tests {
    use automerge::ActorId;
    use libp2p::identity;
    use super::*;
    

    #[tokio::test]
    async fn serialize_deserialize() {
        let keypair = identity::Keypair::generate_ed25519();
        let actor_id = ActorId::random();
        let config = NodeConfig {
            secret: keypair.clone(),
            actor_id: actor_id.clone(),
        };
        let string = toml::to_string(&config).unwrap();
        println!("Parsed config: {}", &string);
        let parsed_config: NodeConfig = toml::from_str(&string).unwrap();
        assert_eq!(keypair.public(), parsed_config.secret.public());
        assert_eq!(actor_id, parsed_config.actor_id);
    }
}
