use std::path::PathBuf;

mod args;
mod device;
mod peer;

pub use args::*;
pub use device::*;
pub use peer::*;

pub trait RunnableCommand {
    async fn run(self);
}