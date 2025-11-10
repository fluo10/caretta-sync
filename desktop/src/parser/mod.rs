//! Provides commandline argument parser
//! 
#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "gui")]
pub use gui::GuiParser;

#[cfg(feature = "server")]
pub use server::ServerParser;