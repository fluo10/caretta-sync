pub const APP_NAME: &str = "caretta-brain";

pub mod config;
#[cfg(feature = "server")]
pub mod engine;
#[cfg(feature = "server")]
pub mod entity;
pub mod error;
pub mod mcp;
#[cfg(test)]
pub mod tests;
pub mod types;
pub mod util;