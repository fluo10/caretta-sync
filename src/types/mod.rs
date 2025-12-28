//! Provide wrapper structs for supporting serde and sea-orm

#[macro_use]
mod macros;

mod util;

mod app_info;
pub use app_info::AppInfo;
#[cfg(feature = "server")]
mod device_key;
#[cfg(feature = "server")]
pub use device_key::*;
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

mod key_parsing_error;
pub use key_parsing_error::*; 

mod token_status;
#[cfg(feature = "desktop")]
mod verbosity;


pub use bytes::*;
pub use doc_ticket::*;

pub use token_status::*;
#[cfg(feature = "desktop")]
pub use verbosity::*;

mod workspace_key;
pub use workspace_key::*;