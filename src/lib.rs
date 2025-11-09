pub use caretta_sync_core as core;
pub use caretta_sync_desktop as desktop;
pub use caretta_sync_service as service;
pub use caretta_sync_ui as ui;
#[cfg(feature = "macros")]
pub mod utils {
    pub use caretta_sync_core::utils::{emptiable::Emptiable, mergeable::Mergeable};
    pub use caretta_sync_macros::{Emptiable, Mergeable};
}
