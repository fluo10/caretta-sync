pub mod rpc;

use clap::Args;
pub use lazy_supplements_core::config::*;

use lazy_supplements_core::utils::{emptiable::Emptiable, mergeable::Mergeable};
use serde::{Deserialize, Serialize};
#[cfg(unix)]
pub use rpc::*;

#[cfg(windows)]
pub use windows::*;

#[derive(Args, Clone, Debug, Deserialize, Emptiable, Mergeable, Serialize)]
pub struct DesktopBaseConfig {
    #[command(flatten)]
    p2p: PartialP2pConfig,
    #[command(flatten)]
    storage: PartialStorageConfig,
    #[command(flatten)]
    rpc: PartialRpcConfig,
}

impl BaseConfig for DesktopBaseConfig {
    fn new() -> Self {
        Self {
            p2p : PartialP2pConfig::empty().with_new_secret(),
            storage: PartialStorageConfig::empty(),
            rpc: PartialRpcConfig::empty().with_unused_port(),
        }
    }
}

impl Into<PartialP2pConfig> for &DesktopBaseConfig {
    fn into(self) -> PartialP2pConfig {
        self.p2p.clone()
    }
}
impl Into<PartialStorageConfig> for &DesktopBaseConfig {
    fn into(self) -> PartialStorageConfig {
        self.storage.clone()
    }
}
impl Into<PartialRpcConfig> for &DesktopBaseConfig {
    fn into(self) -> PartialRpcConfig {
        self.rpc.clone()
    }
}