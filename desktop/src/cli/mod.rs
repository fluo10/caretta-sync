use std::path::PathBuf;

mod args;
mod device;
mod server;

pub use args::*;
pub use device::*;
pub use server::*;

pub trait RunnableCommand {
    async fn run(self);
}