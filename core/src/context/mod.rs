#[cfg(feature = "service")]
mod service;

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "service")]
pub use service::{ServiceContext, ServiceContextExt};

#[cfg(feature = "client")]
pub use client::ClientContext;

#[cfg(feature = "server")]
pub use server::ServerContext;
