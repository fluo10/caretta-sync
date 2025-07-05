pub mod error;
mod storage;
mod p2p;

use std::path::Path;
use crate::{utils::{emptiable::Emptiable, mergeable::Mergeable}};
pub use error::ConfigError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
pub use storage::{StorageConfig, PartialStorageConfig};
pub use p2p::{P2pConfig, PartialP2pConfig};

pub trait Config: TryFrom<Self::PartialConfig>{
    type PartialConfig: PartialConfig<Config = Self>;
}
pub trait PartialConfig: Emptiable + From<Self::Config> + Mergeable {
    type Config: Config<PartialConfig = Self>;

}

pub trait BaseConfig: DeserializeOwned + Serialize {
    fn new() -> Self;
    fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
    fn into_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }
    async fn read_or_create<T>(path: T) -> Result<Self, ConfigError> 
    where
    T: AsRef<Path>
    {
        if !path.as_ref().exists() {
            Self::new().write_to(&path).await?;
        }
        Self::read_from(&path).await
    }
    async fn read_from<T>(path:T) -> Result<Self, ConfigError> 
    where 
    T: AsRef<Path>
    {
        let mut file = File::open(path.as_ref()).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    async fn write_to<T>(&self, path:T) -> Result<(), ConfigError> 
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
    use serde::{Deserialize, Serialize};

    use crate::{tests::test_toml_serialize_deserialize, utils::{emptiable::Emptiable, mergeable::Mergeable}};

    use super::{p2p::{P2pConfig, PartialP2pConfig}, PartialConfig};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestConfig {
        
        p2p: Option<PartialP2pConfig>
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                p2p: Some(PartialP2pConfig::default()),
            }
        }
    }
    impl Emptiable for TestConfig {
        fn empty() -> Self {
            Self {
                p2p: None,
            }
        }

        fn is_empty(&self) -> bool {
            self.p2p.is_none()
        }
    }
    impl Mergeable for TestConfig {
        fn merge(&mut self, other: Self) {
            if let Some(p2p) = other.p2p {
                self.p2p = Some(p2p);
            }
        }
    }
    
    #[tokio::test]
    async fn test_p2p_config_serialize_deserialize() {
        test_toml_serialize_deserialize(TestConfig::empty());
        test_toml_serialize_deserialize(TestConfig::default());
        assert_eq!(TestConfig::empty(), toml::from_str("").unwrap());
        assert_eq!("", &toml::to_string(&TestConfig::empty()).unwrap());
    }
    
}