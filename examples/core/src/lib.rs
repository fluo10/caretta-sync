pub static APP_NAME: &str = "caretta-sync-example";
#[cfg(feature = "server")]
pub mod mcp;
use caretta_sync::types::AppInfo;

#[cfg(feature = "server")]
pub use caretta_sync_example_migration as migration;
