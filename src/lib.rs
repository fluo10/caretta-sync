pub use caretta_sync_core::*;

#[cfg(feature = "bevy")]
pub use caretta_sync_bevy as bevy;
#[cfg(feature = "cli")]
pub use caretta_sync_cli::*;
#[cfg(feature = "mobile")]
pub use caretta_sync_mobile::*;

#[cfg(feature = "macros")]
pub mod utils {
    pub use caretta_sync_core::utils::{
        emptiable::Emptiable, mergeable::Mergeable, runnable::Runnable,
    };
    pub use caretta_sync_macros::{Emptiable, Mergeable, Runnable};
}
