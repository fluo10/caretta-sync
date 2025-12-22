//! Provide wrapper structs for supporting serde and sea-orm

#[macro_use]
mod macros;

mod util;

mod app_info;
pub use app_info::AppInfo;
#[cfg(feature = "server")]
mod author_public_key;
#[cfg(feature = "server")]
mod author_secret_key;
mod bytes;

#[cfg(feature = "server")]
mod app_database;
#[cfg(feature = "server")]
pub use app_database::*;
#[cfg(feature = "server")]
mod database;
#[cfg(feature = "server")]
pub use database::*;

mod doc_ticket;
mod endpoint_public_key;
mod endpoint_secret_key;
#[cfg(feature = "server")]
mod namespace_public_key;
#[cfg(feature = "server")]
mod namespace_secret_key;
mod token_status;
#[cfg(feature = "desktop")]
mod verbosity;

#[cfg(feature = "server")]
pub use author_public_key::*;
#[cfg(feature = "server")]
pub use author_secret_key::*;
pub use bytes::*;
pub use doc_ticket::*;
pub use endpoint_public_key::*;
pub use endpoint_secret_key::*;
#[cfg(feature = "server")]
pub use namespace_public_key::*;
#[cfg(feature = "server")]
pub use namespace_secret_key::*;
pub use token_status::*;
#[cfg(feature = "desktop")]
pub use verbosity::*;
