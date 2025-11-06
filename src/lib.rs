pub use caretta_sync_core::*;

#[cfg(feature = "bevy")]
pub use caretta_sync_bevy as bevy;

#[cfg(feature = "cli")]
pub mod cli {
    pub use caretta_sync_cli::*;
    #[cfg(feature = "macros")]
    pub use caretta_sync_macros::RunnableCommand;
}

#[cfg(feature = "macros")]
pub mod utils {
    pub use caretta_sync_core::utils::{emptiable::Emptiable, mergeable::Mergeable};
    pub use caretta_sync_macros::{Emptiable, Mergeable};
}
