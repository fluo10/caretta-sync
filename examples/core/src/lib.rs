pub static APP_NAME: &str = "caretta-framework-example";
#[cfg(feature = "server")]
pub mod mcp;

#[cfg(feature = "server")]
pub use caretta_framework_example_migration as migration;
