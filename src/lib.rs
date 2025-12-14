pub const APP_NAME: &str = "caretta-brain";

pub mod config;
#[cfg(feature = "engine")]
pub mod engine;
#[cfg(feature = "engine")]
pub mod entity;
pub mod error;
pub mod ipc;
#[cfg(test)]
pub mod tests;
pub mod types;
pub mod util;