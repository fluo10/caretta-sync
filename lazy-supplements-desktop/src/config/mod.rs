#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

pub use lazy_supplements_core::config::*;

use serde::{Deserialize, Serialize};
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
pub use windows::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialDesktopConfig {
    p2p: PartialP2pConfig,
    storage: PartialStorageConfig,
    #[cfg(unix)]
    unix: PartialUnixConfig,
}

impl PartialConfigRoot for PartialDesktopConfig {
    fn new() -> Self {
        Self {
            p2p : PartialP2pConfig::empty().with_new_secret(),
            storage: PartialStorageConfig::empty(),
            unix: PartialUnixConfig::empty(),
        }
    }
}