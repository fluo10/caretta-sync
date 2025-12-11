pub const APP_NAME: &str = "caretta-brain";

pub mod config;
#[cfg(feature = "engine")]
pub mod engine;
pub mod entity;
pub mod error;
pub mod ipc;
#[cfg(feature = "engine")]
pub mod p2p;
#[cfg(test)]
pub mod tests;
pub mod traits;
pub mod types;
pub mod util;