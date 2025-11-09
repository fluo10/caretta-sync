pub mod error;

pub use caretta_sync_core::{
    config,
    context,
    proto,
};    

#[cfg(feature = "service")]
pub use caretta_sync_service::{
    invitation_token,
    model,
    proto_ext,
    service_handler
};
#[cfg(feature = "server")]
pub use caretta_sync_service::server;

#[cfg(feature = "desktop")]
pub use caretta_sync_desktop::{
    args,parsed_config
};

#[cfg(feature = "cli")]
pub use caretta_sync_desktop::subcommand;

pub mod util;
