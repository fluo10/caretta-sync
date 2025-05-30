use std::path::PathBuf;

mod init;
mod node;
mod server;

pub use server::ServerArgs;

pub fn default_config_path() -> PathBuf {
    todo!()
}