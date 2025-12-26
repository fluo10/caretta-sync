#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "desktop")]
pub mod args;

pub mod config;
#[cfg(feature = "server")]
pub mod entity;
pub mod error;
pub mod mcp;

#[cfg(test)]
pub mod tests;
pub mod types;
pub mod util;
