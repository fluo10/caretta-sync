#[cfg(feature = "desktop")]
pub mod args;

pub mod config;
#[cfg(feature = "server")]
pub mod entity;
pub mod error;
pub mod mcp;
#[cfg(feature = "desktop")]
pub mod parsed_config;
#[cfg(feature = "desktop")]
pub mod parser;

#[cfg(feature = "desktop-cli")]
pub mod subcommand;

#[cfg(test)]
pub mod tests;
pub mod types;
pub mod util;
