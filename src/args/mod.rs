pub mod option;

#[cfg(any(feature = "desktop-gui", feature = "desktop-server"))]
pub mod parser;

#[cfg(feature = "desktop-cli")]
pub mod subcommand;