mod log;
#[cfg(feature = "server")]
mod p2p;
#[cfg(feature = "desktop")]
mod mcp;
#[cfg(feature = "server")]
mod storage;

pub use log::LogConfig;
#[cfg(feature = "server")]
pub use p2p::P2pConfig;
#[cfg(feature = "desktop")]
pub use mcp::McpConfig;
#[cfg(feature = "server")]
pub use storage::StorageConfig;
