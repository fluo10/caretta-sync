use std::path::PathBuf;

mod connect;
mod init;
mod server;

pub use server::ServerArgs;

pub fn default_config_path() -> PathBuf {
    todo!()
}