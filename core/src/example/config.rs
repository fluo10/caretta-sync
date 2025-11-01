#[cfg(feature="cli")]
use clap::Args;

use serde::{Deserialize, Serialize};

use crate::{config::{ParsedConfig, PartialP2pConfig, PartialRpcConfig, PartialStorageConfig}, utils::{emptiable::Emptiable, mergeable::Mergeable}};


#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExampleParsedConfig {
    #[cfg_attr(feature = "cli", command(flatten))]
    pub p2p: Option<PartialP2pConfig>,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub storage: Option<PartialStorageConfig>,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub rpc: Option<PartialRpcConfig>,
}

impl ParsedConfig for ExampleParsedConfig {
    fn partial_storage_config(&self) -> Option<&PartialStorageConfig> {
        self.storage.as_ref()
    }

    fn partial_p2p_config(&self) -> Option<&PartialP2pConfig> {
        self.p2p.as_ref()
    }

    fn partial_rpc_config(&self) -> Option<&PartialRpcConfig> {
        self.rpc.as_ref()
    }

    fn default(app_name: &'static str) -> Self {
        todo!()
    }
}

impl Emptiable for ExampleParsedConfig {
    fn empty() -> Self {
        Self {
            p2p: None,
            storage: None,
            rpc: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.rpc.is_empty() && self.storage.is_empty()
    }
}

impl Mergeable for ExampleParsedConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.rpc.merge(other.rpc);
        self.storage.merge(other.storage);
    }
}
