pub mod error;
pub mod ipc;
pub use caretta_sync_core::{config, context};

#[cfg(feature = "server")]
pub use caretta_sync_service::server;
#[cfg(feature = "server")]
pub use caretta_sync_service::{
    local_data,
    synced_data,
};

#[cfg(feature = "desktop")]
pub use caretta_sync_desktop::{args, parsed_config};
#[cfg(any(feature = "gui", feature = "server"))]
pub use caretta_sync_desktop::{parser};

#[cfg(feature = "cli")]
pub use caretta_sync_desktop::subcommand;

pub mod util;
