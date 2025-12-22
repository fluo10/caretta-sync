#[cfg(feature = "desktop-client")]
mod client;
#[cfg(feature = "desktop-client")]
pub use client::*;

mod log;
pub use log::LogConfig;

#[cfg(feature = "server")]
mod p2p;
#[cfg(feature = "server")]
pub use p2p::P2pConfig;

#[cfg(feature = "desktop")]
mod mcp;
#[cfg(feature = "desktop")]
pub use mcp::McpConfig;

#[cfg(feature = "mobile")]
mod mobile;
#[cfg(feature = "mobile")]
pub use mobile::*;

#[cfg(feature = "desktop-server")]
mod server;
#[cfg(feature = "desktop-server")]
pub use server::*;

#[cfg(feature = "server")]
mod server_ext;
#[cfg(feature = "server")]
pub use server_ext::*;

#[cfg(feature = "server")]
mod storage;
#[cfg(feature = "server")]
pub use storage::StorageConfig;
